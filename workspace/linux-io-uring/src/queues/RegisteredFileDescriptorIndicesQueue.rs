// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registered file descriptors.
#[derive(Debug)]
pub struct RegisteredFileDescriptorIndicesQueue(LargeRingQueue<RegisteredFileDescriptorIndex>);

impl RegisteredFileDescriptorIndicesQueue
{
	/// New instance.
	pub fn new(defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		let mut this = Self(LargeRingQueue::new(RegisteredFileDescriptorIndex::ExclusiveMaximum, defaults, 0, false)?);
		
		for index in 0 .. RegisteredFileDescriptorIndex::ExclusiveMaximum
		{
			let index = RegisteredFileDescriptorIndex(index as u32);
			this.0.enqueue_unchecked(index)
		}
		
		Ok(this)
	}
	
	/// Enqueues and copies.
	///
	/// Returns `false` if enqueue failed because the queue is full.
	#[inline(always)]
	pub fn enqueue_checked(&mut self, value: RegisteredFileDescriptorIndex) -> bool
	{
		self.0.enqueue_checked(value)
	}
	
	/// Enqueues and copies without checking for capacity.
	#[inline(always)]
	pub unsafe fn enqueue_unchecked(&mut self, value: RegisteredFileDescriptorIndex)
	{
		self.0.enqueue_unchecked(value)
	}
	
	/// Dequeues and copies without checking for capacity.
	#[inline(always)]
	pub fn dequeue(&mut self) -> Option<RegisteredFileDescriptorIndex>
	{
		self.0.dequeue()
	}
}
