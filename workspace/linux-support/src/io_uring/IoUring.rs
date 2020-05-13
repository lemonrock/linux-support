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
	
	/// The buffers associated with the iovecs will be locked in memory and charged against the user's `RLIMIT_MEMLOCK` resource limit.
	///
	/// See `getrlimit(2)` for more information.
	/// Additionally, there is a size limit of 1GiB per buffer.
	/// Currently, the buffers must be anonymous, non-file-backed memory, such as that returned by `malloc(3)` or `mmap(2)` with the `MAP_ANONYMOUS` flag set.
	/// It is expected that this limitation will be lifted in the future.
	/// Huge pages are supported as well.
	/// Note that the entire huge page will be pinned in the kernel, even if only a portion of it is used.
	/// NOTE: Relies on `iovec` having the same layout as a Rust slice.
	/// `buffers` needs to be anonymous memory from either `mmap()`, `malloc()` or a memory file descriptor.
	/// It lives until unregistered.
	///
	/// After a successful call, the supplied buffers are mapped into the kernel and eligible for I/O.
	/// To make use of them, the application must specify the `IORING_OP_READ_FIXED` or `IORING_OP_WRITE_FIXED` opcodes in the submission queue entry (see the `struct io_uring_sqe` definition in `io_uring_enter(2)`), and set the `buf_index` field to the desired buffer index.
	/// The memory range described by the submission queue entry's `addr` and `len` fields must fall within the indexed buffer.
	///
	/// It is perfectly valid to setup a large buffer and then only use part of it for an I/O, as long as the range is within the originally mapped region.
	///
	/// An application can increase or decrease the size or number of registered buffers by first unregistering the existing buffers, and then issuing a new call to `io_uring_register()` with the new buffers.
	///
	/// Note that registering buffers will wait for the ring to idle.
	/// If the application currently has requests in-flight, the registration will wait for those to finish before proceeding.
	///
	/// An application need not unregister buffers explicitly before shutting down the io_uring instance.
	///
	/// This is a system call.
	///
	/// Since Linux 5.1.
	#[inline(always)]
	pub fn register_buffers(&self, buffers: &[&mut [u8]]) -> io::Result<()>
	{
		self.io_uring_file_descriptor.register_buffers(buffers)
	}
	
	/// Unregisters all buffers.
	///
	/// All previously registered buffers associated with the io_uring instance will be released.
	///
	/// This is a system call.
	///
	/// Since Linux 5.1.
	#[inline(always)]
	pub fn unregister_all_buffers(&self) -> io::Result<()>
	{
		self.io_uring_file_descriptor.unregister_all_buffers()
	}
	
	/// Register files for I/O.
	///
	/// To make use of the registered files, the `IOSQE_FIXED_FILE` flag must be set in the `flags` member of the struct `io_uring_sqe`, and the `fd` member is set to the index of the file in the file descriptor array.
	///
	/// The `files_descriptors` set may be sparse (ie not all file descriptors need to be valid, open files or the like).
	/// See `IORING_REGISTER_FILES_UPDATE` for how to update files in place.
	///
	/// Note that registering files will wait for the ring to idle.
	/// If the application currently has requests in-flight, the registration will wait for those to finish before proceeding.
	/// See `IORING_REGISTER_FILES_UPDATE` for how to update an existing set without that limitation.
	///
	/// Files are automatically unregistered when the io_uring instance is torn down.
	///
	/// This is a system call.
	///
	/// Since Linux 5.1.
	#[inline(always)]
	pub fn register_file_descriptors(&self, file_descriptors: &[SupportedFileDescriptor]) -> io::Result<()>
	{
		self.io_uring_file_descriptor.register_file_descriptors(file_descriptors)
	}
	
	/// Unregisters all file descriptors.
	///
	/// All previously registered file descriptors associated with the io_uring instance will be released.
	///
	/// All registered file descriptors are automatically released on drop.
	///
	/// This is a system call.
	///
	/// Since Linux 5.1.
	#[inline(always)]
	pub fn unregister_all_file_descriptors(&self) -> io::Result<()>
	{
		self.io_uring_file_descriptor.unregister_all_file_descriptors()
	}
	
	/// Update registered file descriptors in-place.
	///
	/// All registered file descriptors are automatically released on drop.
	///
	/// Prefer `IORING_OP::IORING_OP_FILES_UPDATE` as this can eliminate a system call.
	///
	/// This is a system call.
	///
	/// Since Linux 5.5.
	#[allow(deprecated)]
	#[deprecated]
	#[inline(always)]
	pub fn replace_some_registered_file_descriptors(&self, replace_with_files_descriptors: &[SupportedFileDescriptor], starting_from_index_inclusive: u32) -> Result<(), ()>
	{
		self.io_uring_file_descriptor.replace_some_registered_file_descriptors(replace_with_files_descriptors, starting_from_index_inclusive)
	}
	
	/// It's possible to use an `EventFileDescriptor` to get notified of completion events on an io_uring instance.
	///
	/// The event file descriptor is automatically released on drop.
	///
	/// This is a system call.
	///
	/// Since Linux 5.2.
	#[inline(always)]
	pub fn register_event_file_descriptor_for_all_notifications(&self, event_file_descriptor: &EventFileDescriptor) -> Result<(), bool>
	{
		self.io_uring_file_descriptor.register_event_file_descriptor_for_all_notifications(event_file_descriptor)
	}
	
	/// It's possible to use an `EventFileDescriptor` to get notified of completion events on an io_uring instance.
	///
	/// The event file descriptor is automatically released on drop.
	///
	/// In this case, notifications are only received for completion events that didn't complete immediately (ie those that didn't complete inline when being submitted).
	///
	/// This is a system call.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn register_event_file_descriptor_for_only_asynchronous_notifications(&self, event_file_descriptor: &EventFileDescriptor) -> Result<(), bool>
	{
		self.io_uring_file_descriptor.register_event_file_descriptor_for_only_asynchronous_notifications(event_file_descriptor)
	}
	
	/// Unregister an event file desscriptor.
	///
	/// The event file descriptor is automatically released on drop.
	///
	/// This is a system call.
	///
	/// Since Linux 5.2.
	#[inline(always)]
	pub fn unregister_event_file_descriptor_for_notifications(&self) -> Result<(), bool>
	{
		self.io_uring_file_descriptor.unregister_event_file_descriptor_for_notifications()
	}
	
	/// Probes for supported operations.
	///
	/// Fails if Linux kernel is out-of-memory.
	///
	/// This is a system call.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn probe_for_supported_operations(&self) -> Result<BTreeSet<IORING_OP>, ()>
	{
		self.io_uring_file_descriptor.probe_for_supported_operations()
	}
	
	/// This operation registers credentials of the running application with io_uring, and returns a credential identifier associated with these credentials.
	///
	/// Applications wishing to share a ring between separate users or processes can pass in this credential identifier in the `SubmissionQueueEntry` `personality` field.
	/// If set, that particular `SubmissionQueueEntry` will be issued with these credentials.
	///
	/// Returns `Err(())` if Linux kernel is out of memory (try again).
	///
	/// This is a system call.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn register_personality(&self) -> Result<PersonalityCredentialsIdentifier, ()>
	{
		self.io_uring_file_descriptor.register_personality()
	}
	
	/// Unregisters a previously registered personality with io_uring.
	///
	/// This is a system call.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn unregister_personality(&self, personality_credentials_identifier: PersonalityCredentialsIdentifier)
	{
		self.io_uring_file_descriptor.unregister_personality(personality_credentials_identifier)
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
	pub fn completion_queue_ring_overflow(&self) -> u32
	{
		self.completion_queue_ring.overflow()
	}
	
	/// Number of dropped events.
	#[inline(always)]
	pub fn submission_queue_dropped(&self) -> u32
	{
		self.submission_queue_ring.dropped()
	}

	/// This is slightly expensive and can change immediately after being called for the case of `false` (ie can immediately become `true`).
	#[inline]
	pub fn completion_queue_ring_is_empty(&self) -> bool
	{
		self.completion_queue_ring.is_empty()
	}

	/// This is slightly expensive and can change immediately after being called for the case of `true` (ie can immediately become `false`).
	#[inline]
	pub fn completion_queue_ring_is_full(&self) -> bool
	{
		self.completion_queue_ring.is_full()
	}

	/// This is slightly expensive and can change immediately after being called; it can get larger but never smaller.
	#[inline(always)]
	pub fn completion_queue_ring_available(&self) -> u32
	{
		self.completion_queue_ring.available()
	}

	/// This is slightly expensive and can change immediately after being called.
	#[inline(always)]
	pub fn completion_queue_ring_length(&self) -> u32
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
