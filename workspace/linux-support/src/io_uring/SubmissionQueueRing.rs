// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pointers are pointers to values in `submission_queue_and_completion_queue: MappedMemory`.
///
/// Pointers are only valid whilst this `submission_queue_and_completion_queue` exists.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SubmissionQueueRing
{
	head: NonNull<AtomicU32>,

	tail: NonNull<AtomicU32>,

	ring_mask: NonNull<u32>,

	ring_entries: NonNull<u32>,

	/// Actually of type 'SubmissionQueueRingFlags'.
	flags: NonNull<AtomicU32>,

	dropped: NonNull<AtomicU32>,

	/// A fixed-size list of zero-based indices.
	///
	/// Each index refers to the location of a `io_uring_sqe` in `submission_queue_entries`.
	array: NonNull<u32>,

	submission_queue_entries: MappedMemory,
}

impl SubmissionQueueRing
{
	#[inline(always)]
	pub(crate) fn new(submission_queue_and_completion_queue: &MappedMemory, submission_queue_offsets: &io_sqring_offsets, submission_queue_entries: MappedMemory) -> Self
	{
		let memory = submission_queue_and_completion_queue.virtual_address();
		let this = Self
		{
			head: memory.pointer_to::<AtomicU32>(submission_queue_offsets.head as usize),
			tail: memory.pointer_to::<AtomicU32>(submission_queue_offsets.head as usize),
			ring_mask: memory.pointer_to::<u32>(submission_queue_offsets.head as usize),
			ring_entries: memory.pointer_to::<u32>(submission_queue_offsets.head as usize),
			flags: memory.pointer_to::<AtomicU32>(submission_queue_offsets.head as usize),
			dropped: memory.pointer_to::<AtomicU32>(submission_queue_offsets.head as usize),
			array: memory.pointer_to::<u32>(submission_queue_offsets.head as usize),
			submission_queue_entries,
		};
		this.set_up_one_to_one_mapping_from_array_to_submission_queue_entry();
		this
	}

	#[inline(always)]
	fn set_up_one_to_one_mapping_from_array_to_submission_queue_entry(&self)
	{
		for index in 0 .. self.array_length()
		{
			let pointer = self.array_element(index).as_ptr();

			let submission_queue_entry_index = index;
			unsafe { pointer.write_volatile(submission_queue_entry_index) };
		}
	}

	/// `add_entries` should return `true` when there are no more entries to push.
	///
	/// Returns an error if full but `add_entries` had more entries to push.
	/// In this case, one can immediately retry this method as the Linux kernel may have updated `head` is using a kernel thread for submission queue polling.
	#[inline(always)]
	pub(crate) fn push_submission_queue_entries<'add_entries, AddEntries: FnMut(SubmissionQueueEntry) -> bool>(&self, add_entries: &'add_entries mut AddEntries) -> Result<(), &'add_entries mut AddEntries>
	{
		let head = self.head_atomically();
		let mut tail = self.tail_non_atomically();
		let ring_mask = self.ring_mask();

		let mut stop_pushing_entries;
		loop
		{
			if unlikely!(self.is_full_internal(head, tail))
			{
				return Err(add_entries)
			}

			stop_pushing_entries = add_entries(self.next_submission_queue_entry(tail, ring_mask));
			tail = tail.wrapping_add(1);

			if stop_pushing_entries
			{
				break
			}
		}

		self.store_tail_atomically(tail);
		Ok(())
	}

	#[inline(always)]
	fn next_submission_queue_entry(&self, tail: u32, ring_mask: u32) -> SubmissionQueueEntry
	{
		SubmissionQueueEntry(self.submission_queue_entries.virtual_address().pointer_to((tail & ring_mask) as usize))
	}

	#[inline(always)]
	pub(crate) fn needs_to_wake_up_kernel_submission_queue_poll_thread(&self) -> bool
	{
		self.flags().contains(SubmissionQueueRingFlags::NeedsIoUringEnterWakeUp)
	}

	/// This is slightly expensive and can change immediately after being called for the case of `false` (ie can immediately become `true`).
	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.length() == 0
	}

	/// This is slightly expensive and can change immediately after being called for the case of `true` (ie can immediately become `false`).
	#[inline(always)]
	pub(crate) fn is_full(&self) -> bool
	{
		self.length() == self.array_length()
	}

	/// This is slightly expensive and can change immediately after being called; it can get larger but never smaller.
	#[inline(always)]
	pub(crate) fn available(&self) -> u32
	{
		self.array_length() - self.length()
	}

	/// This is slightly expensive and can change immediately after being called.
	#[inline(always)]
	pub(crate) fn length(&self) -> u32
	{
		let head = self.head_atomically();
		let tail = self.tail_non_atomically();

		Self::length_internal(head, tail)
	}

	#[inline(always)]
	fn is_full_internal(&self, head: u32, tail: u32) -> bool
	{
		Self::length_internal(head, tail) == self.array_length()
	}

	#[inline(always)]
	fn length_internal(head: u32, tail: u32) -> u32
	{
		tail.wrapping_sub(head)
	}

	#[inline(always)]
	fn head_atomically(&self) -> u32
	{
		self.head.load_acquire()
	}

	#[inline(always)]
	fn store_tail_atomically(&self, new_tail: u32)
	{
		self.tail.store_release(new_tail)
	}

	#[inline(always)]
	fn tail_non_atomically(&self) -> u32
	{
		self.tail.load_non_atomically()
	}

	#[inline(always)]
	fn ring_mask(&self) -> u32
	{
		self.ring_mask.unsynchronized_value()
	}

	/// `array.len()` or `capacity()`.
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.ring_entries.unsynchronized_value()
	}

	#[inline(always)]
	fn array_element(&self, index: u32) -> NonNull<u32>
	{
		unsafe { NonNull::new_unchecked(self.array.as_ptr().add(index as usize)) }
	}

	#[inline(always)]
	fn flags(&self) -> SubmissionQueueRingFlags
	{
		unsafe { transmute(self.flags.load_acquire()) }
	}

	#[inline(always)]
	fn dropped(&self) -> u32
	{
		self.dropped.load_acquire()
	}
}
