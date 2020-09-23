// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A queue of unused frames.
///
/// Frame numbers must be unique; this is not enforced.
#[derive(Debug)]
pub(crate) struct MultipleProducerMultipleConsumerAlignedFreeFrameQueue(ArrayQueue<AlignedFrameNumber>);

unsafe impl Send for MultipleProducerMultipleConsumerAlignedFreeFrameQueue
{
}

unsafe impl Sync for MultipleProducerMultipleConsumerAlignedFreeFrameQueue
{
}

impl FreeFrameQueue for MultipleProducerMultipleConsumerAlignedFreeFrameQueue
{
	type CS = AlignedChunkSize;
	
	#[inline(always)]
	fn new(number_of_chunks: NonZeroU32, _user_memory: &MappedMemory) -> Self
	{
		let this = Self(ArrayQueue::new(number_of_chunks.get() as usize));
		
		this.populate();
		
		this
	}
	
	#[inline(always)]
	fn push(&self, newly_freed_frame_identifier: AlignedFrameNumber)
	{
		debug_assert!(newly_freed_frame_identifier < self.exclusive_maximum_aligned_frame_number());
		
		self.0.push(newly_freed_frame_identifier).expect("Pushed the same frame_number more than once!")
	}
	
	#[inline(always)]
	fn pop(&self) -> Option<AlignedFrameNumber>
	{
		self.0.pop().ok()
	}
}

impl MultipleProducerMultipleConsumerAlignedFreeFrameQueue
{
	#[inline(always)]
	fn populate(&self)
	{
		for frame_identifier in AlignedFrameNumber::InclusiveMinimum .. self.exclusive_maximum_aligned_frame_number()
		{
			self.push(frame_identifier)
		}
	}
	
	#[inline(always)]
	fn exclusive_maximum_aligned_frame_number(&self) -> AlignedFrameNumber
	{
		AlignedFrameNumber(self.0.capacity() as u32)
	}
}
