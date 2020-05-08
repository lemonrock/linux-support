// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct CompletionQueueRing
{
	head: NonNull<AtomicU32>,

	tail: NonNull<AtomicU32>,

	ring_mask: NonNull<u32>,

	ring_entries: NonNull<u32>,

	overflow: NonNull<AtomicU32>,

	completion_queue_entries: NonNull<io_uring_cqe>,
}

impl CompletionQueueRing
{
	#[inline(always)]
	fn new(submission_queue_and_completion_queue: &MappedMemory, completion_queue_offsets: &io_cqring_offsets) -> Self
	{
		let memory = submission_queue_and_completion_queue.virtual_address();
		Self
		{
			head: memory.pointer_to::<AtomicU32>(completion_queue_offsets.head as usize),
			tail: memory.pointer_to::<AtomicU32>(completion_queue_offsets.head as usize),
			ring_mask: memory.pointer_to::<u32>(completion_queue_offsets.head as usize),
			ring_entries: memory.pointer_to::<u32>(completion_queue_offsets.head as usize),
			overflow: memory.pointer_to::<AtomicU32>(completion_queue_offsets.head as usize),
			completion_queue_entries: memory.pointer_to::<io_uring_cqe>(completion_queue_offsets.cqes as usize),
		}
	}

    /// Number of dropped events because the completion queue was full.
	#[inline(always)]
	pub(crate) fn overflow(&self) -> u32
	{
		self.overflow.load_acquire()
    }

	/// This is slightly expensive and can change immediately after being called for the case of `false` (ie can immediately become `true`).
	#[inline]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.length() == 0
	}

	/// This is slightly expensive and can change immediately after being called for the case of `true` (ie can immediately become `false`).
	#[inline]
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
		let head = self.head.load_non_atomically();
		let tail = self.tail.load_acquire();
		tail.wrapping_sub(head)
    }

	/// `array.len()` or `capacity()`.
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.ring_entries.unsynchronized_value()
	}
}
