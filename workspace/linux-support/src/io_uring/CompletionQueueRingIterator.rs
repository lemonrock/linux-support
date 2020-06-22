// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterates over completion queue.
#[derive(Debug)]
pub struct CompletionQueueRingIterator<'a>
{
	head: u32,
	tail: u32,
	ring_mask: u32,
	ring_entries: u32,
	completion_queue_ring: &'a mut CompletionQueueRing,
}

impl ExactSizeIterator for CompletionQueueRingIterator<'_>
{
	#[inline(always)]
	fn len(&self) -> usize
	{
		self.tail.wrapping_sub(self.head) as usize
	}
}

impl<'a> Iterator for CompletionQueueRingIterator<'a>
{
	type Item = CompletionQueueEntry<'a>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.is_empty_()
		{
			None
		}
		else
		{
			let pointer = self.completion_queue_ring.completion_queue_entries.as_ptr();
			let index = (self.head & self.ring_mask) as usize;
			let completion_queue_entry = unsafe { & * pointer.add(index) };

			self.head = self.head.wrapping_add(1);

			Some(CompletionQueueEntry(completion_queue_entry))
		}
	}
}

impl<'a> Drop for CompletionQueueRingIterator<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.tell_linux_kernel_all_completion_entries_iterated_so_far_are_finished_with()
	}
}

impl<'a> CompletionQueueRingIterator<'a>
{
	#[inline(always)]
	fn new(completion_queue_ring: &'a mut CompletionQueueRing) -> Self
	{
		Self
		{
			head: completion_queue_ring.head.load_non_atomically(),
			tail: completion_queue_ring.tail.load_acquire(),
			ring_mask: completion_queue_ring.ring_mask.unsynchronized_value(),
			ring_entries: completion_queue_ring.ring_entries.unsynchronized_value(),
			completion_queue_ring,
		}
	}
	
	/// Tells Linux kernel all completion entries iterated so far are finished with.
	#[inline(always)]
	pub fn tell_linux_kernel_all_completion_entries_iterated_so_far_are_finished_with(&mut self)
	{
		self.store_head();
	}

	/// Tells Linux kernel all completion entries iterated so far are finished with.
	/// Updates to get new list of completion entries available.
	#[inline(always)]
	pub fn synchronize(&mut self)
	{
		self.tell_linux_kernel_all_completion_entries_iterated_so_far_are_finished_with();
		self.tail = self.completion_queue_ring.tail.load_acquire()
	}

	/// Is empty?
	///
	/// Collides with `is_empty()` in `ExactSizeIterator`.
	#[inline(always)]
	pub fn is_empty_(&self) -> bool
	{
		self.head == self.tail
	}

	/// Is full?
	#[inline(always)]
	pub fn is_full(&self) -> bool
	{
		(self.len() as u32) == self.array_length()
	}

	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.ring_entries
	}

	#[inline(always)]
	fn store_head(&self)
	{
		self.completion_queue_ring.head.store_release(self.head)
	}
}
