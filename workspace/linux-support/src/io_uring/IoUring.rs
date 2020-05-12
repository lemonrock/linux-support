// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An io_uring instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IoUring<'a>
{
	io_uring_file_descriptor: IoUringFileDescriptor,
	submission_queue_ring: SubmissionQueueRing,
	completion_queue_ring: CompletionQueueRing,
	submission_queue_and_completion_queue: MappedMemory,
	shared_work_queue: Option<&'a IoUringFileDescriptor>,
}

impl<'a> IoUring<'a>
{
	/// Creates a new instance.
	///
	/// This is an expensive operation involving several system calls.
	#[inline(always)]
	pub fn new(defaults: &DefaultPageSizeAndHugePageSizes, number_of_submission_queue_entries: NonZeroU16, number_of_completion_queue_entries: Option<NonZeroU32>, kernel_submission_queue_thread_configuration: Option<&LinuxKernelSubmissionQueuePollingThreadConfiguration>, shared_work_queue: Option<&'a IoUring>) -> Result<Self, IoUringCreationError>
	{
		let shared_work_queue = shared_work_queue.map(|io_uring| &io_uring.io_uring_file_descriptor);
		let (io_uring_file_descriptor, parameters) = IoUringFileDescriptor::new(number_of_submission_queue_entries, number_of_completion_queue_entries, kernel_submission_queue_thread_configuration, shared_work_queue).map_err(IoUringCreationError::CouldNotCreateIoUringFileDescriptor)?;

		Self::construct(io_uring_file_descriptor, &parameters, defaults, shared_work_queue)
	}

	/// Initiates asynchronous I/O.
	///
	/// Returns immediately.
	///
	/// Does not even make a syscall if the Linux kernel thread doing submission polling doesn't need to be woken up!
	///
	/// Returns the number of submission queue entries (SQEs) successfully consumed.
	#[inline(always)]
	pub fn initiate_asynchronous_io(&self) -> Result<u32, SubmitError>
	{
		self.io_uring_file_descriptor.enter(None, &self.submission_queue_ring, None)
	}

	/// Initiates asynchronous I/O.
	///
	/// Waits until approximately `minimum_wanted_to_complete` completions have occurred.
	///
	/// Returns the number of submission queue entries (SQEs) successfully consumed.
	#[inline(always)]
	pub fn initiate_asynchronous_io_and_wait(&self, minimum_wanted_to_complete: NonZeroU32) -> Result<u32, SubmitError>
	{
		self.io_uring_file_descriptor.enter(Some(minimum_wanted_to_complete), &self.submission_queue_ring, None)
	}

	/// This returns the functor if the submission queue filled up.
	///
	/// Retry after calling `self.initiate_asynchronous_io()`.
	#[inline(always)]
	pub fn push_submission_queue_entries<'add_entries, AddEntries: FnMut(SubmissionQueueEntry) -> bool>(&self, add_entries: &'add_entries mut AddEntries) -> Result<(), &'add_entries mut AddEntries>
	{
		self.submission_queue_ring.push_submission_queue_entries(add_entries, self.io_uring_file_descriptor.using_kernel_submission_queue_poll, self.io_uring_file_descriptor.using_io_poll)
	}

	/// Call this regularly to clear down the completion queue.
	///
	/// Dropping the `CompletionQueueRingIterator` frees all completion queue entries.
	#[inline(always)]
	pub fn current_completion_queue_entries(&mut self) -> CompletionQueueRingIterator
	{
		CompletionQueueRingIterator::new(&mut self.completion_queue_ring)
	}

	/// This is slightly expensive and can change immediately after being called for the case of `false` (ie can immediately become `true`).
	#[inline(always)]
	pub fn submission_queue_ring_is_empty(&self) -> bool
	{
		self.submission_queue_ring.is_empty()
	}

	/// This is slightly expensive and can change immediately after being called for the case of `true` (ie can immediately become `false`).
	#[inline(always)]
	pub fn submission_queue_ring_is_full(&self) -> bool
	{
		self.submission_queue_ring.is_full()
	}

	/// This is slightly expensive and can change immediately after being called; it can get larger but never smaller.
	#[inline(always)]
	pub fn submission_queue_ring_available(&self) -> u32
	{
		self.submission_queue_ring.available()
	}

	/// Number of dropped events because the completion queue was full.
	#[inline(always)]
	pub(crate) fn completion_queue_ring_overflow(&self) -> u32
	{
		self.completion_queue_ring.overflow()
	}

	/// This is slightly expensive and can change immediately after being called for the case of `false` (ie can immediately become `true`).
	#[inline]
	pub(crate) fn completion_queue_ring_is_empty(&self) -> bool
	{
		self.completion_queue_ring.is_empty()
	}

	/// This is slightly expensive and can change immediately after being called for the case of `true` (ie can immediately become `false`).
	#[inline]
	pub(crate) fn completion_queue_ring_is_full(&self) -> bool
	{
		self.completion_queue_ring.is_full()
	}

	/// This is slightly expensive and can change immediately after being called; it can get larger but never smaller.
	#[inline(always)]
	pub(crate) fn completion_queue_ring_available(&self) -> u32
	{
		self.completion_queue_ring.available()
	}

	/// This is slightly expensive and can change immediately after being called.
	#[inline(always)]
	pub(crate) fn completion_queue_ring_length(&self) -> u32
	{
		self.completion_queue_ring.length()
	}

	#[inline(always)]
	fn construct(io_uring_file_descriptor: IoUringFileDescriptor, parameters: &io_uring_params, defaults: &DefaultPageSizeAndHugePageSizes, shared_work_queue: Option<&'a IoUringFileDescriptor>) -> Result<Self, IoUringCreationError>
	{
		let actual_number_of_submission_queue_entries = parameters.sq_entries;
		let actual_number_of_completion_queue_entries = parameters.cq_entries;
		let submission_queue_offsets = &parameters.sq_off;
		let completion_queue_offsets = &parameters.cq_off;

		let (submission_queue_entries, _submission_queue_entries_size) = Self::submission_queue_entries_mmap(&io_uring_file_descriptor, actual_number_of_submission_queue_entries, defaults)?;
		let (submission_queue_and_completion_queue, _submission_queue_size, _completion_queue_size) = Self::submission_queue_and_completion_queue_mmap(&io_uring_file_descriptor, actual_number_of_submission_queue_entries, actual_number_of_completion_queue_entries, submission_queue_offsets, completion_queue_offsets, defaults)?;

		Ok
		(
			Self
			{
				io_uring_file_descriptor,
				submission_queue_ring: SubmissionQueueRing::new(&submission_queue_and_completion_queue, submission_queue_offsets, submission_queue_entries),
				completion_queue_ring: CompletionQueueRing::new(&submission_queue_and_completion_queue, completion_queue_offsets),
				submission_queue_and_completion_queue,
				shared_work_queue
			}
		)
	}

	fn submission_queue_entries_mmap(io_uring_file_descriptor: &IoUringFileDescriptor, actual_number_of_submission_queue_entries: u32, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(MappedMemory, NonZeroU64), IoUringCreationError>
	{
		let submission_queue_entries_size = Self::size::<io_uring_sqe>(0, actual_number_of_submission_queue_entries);

		let mapped_memory = Self::map_memory(io_uring_file_descriptor, submission_queue_entries_size, IORING_OFF_SQES, defaults)?;
		Ok((mapped_memory, submission_queue_entries_size))
	}

	fn submission_queue_and_completion_queue_mmap(io_uring_file_descriptor: &IoUringFileDescriptor, actual_number_of_submission_queue_entries: u32, actual_number_of_completion_queue_entries: u32, submission_queue_offsets: &io_sqring_offsets, completion_queue_offsets: &io_cqring_offsets, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(MappedMemory, NonZeroU64, NonZeroU64), IoUringCreationError>
	{
		let submission_queue_size = Self::size::<u32>(submission_queue_offsets.array, actual_number_of_submission_queue_entries);
		let completion_queue_size = Self::size::<io_uring_cqe>(completion_queue_offsets.cqes, actual_number_of_completion_queue_entries);
		// Assumes that parameters' feature flags contains `ParametersFeatureFlags::SingleMMap`.
		let size = max(submission_queue_size, completion_queue_size);

		let mapped_memory = Self::map_memory(io_uring_file_descriptor, size, IORING_OFF_SQ_RING, defaults)?;
		Ok((mapped_memory, submission_queue_size, completion_queue_size))
	}

	#[inline(always)]
	fn size<T: Sized>(left: u32, number_of_entries: u32) -> NonZeroU64
	{
		unsafe { NonZeroU64::new_unchecked((left as u64) + ((number_of_entries * (size_of::<T>() as u32)) as u64)) }
	}

	#[inline(always)]
	fn map_memory(io_uring_file_descriptor: &IoUringFileDescriptor, size: NonZeroU64, offset: u64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<MappedMemory, IoUringCreationError>
	{
		const OneMegabyte: usize = 1024 * 1024;
		let huge_memory_page_size = defaults.best_fit_huge_page_size_if_any(size.get() as usize, OneMegabyte).map(|huge_page_size| Some(huge_page_size));
		let mapped_memory = MappedMemory::from_file(io_uring_file_descriptor, offset, size, AddressHint::any(), Protection::ReadWrite, Sharing::Shared, huge_memory_page_size, true, false, defaults)?;

		mapped_memory.advise(MemoryAdvice::DontFork).map_err(IoUringCreationError::CouldNotAdviseDontFork)?;

		Ok(mapped_memory)
	}
}
