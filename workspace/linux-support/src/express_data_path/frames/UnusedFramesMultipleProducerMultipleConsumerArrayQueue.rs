// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A queue of unused frames.
///
/// Frame numbers must be unique; this is not enforced.
#[derive(Debug)]
pub(crate) struct UnusedFramesMultipleProducerMultipleConsumerArrayQueue(ArrayQueue<AlignedFrameNumber>);

unsafe impl Send for UnusedFramesMultipleProducerMultipleConsumerArrayQueue
{
}

unsafe impl Sync for UnusedFramesMultipleProducerMultipleConsumerArrayQueue
{
}

impl UnusedFramesMultipleProducerMultipleConsumerArrayQueue
{
	#[inline(always)]
	pub(crate) fn new(number_of_frames: NonZeroU32, number_of_frames_initially_gifted_to_the_linux_kernel: Option<NonZeroU32>) -> Self
	{
		let number_of_frames = number_of_frames.get();
		let this = Self(ArrayQueue::new(number_of_frames as usize));
		
		this.populate(number_of_frames, number_of_frames_initially_gifted_to_the_linux_kernel);
		
		this
	}
	
	#[inline(always)]
	fn populate(&self, number_of_frames: u32, number_of_frames_initially_gifted_to_the_linux_kernel: Option<NonZeroU32>)
	{
		if let Some(number_of_frames_initially_gifted_to_the_linux_kernel) = number_of_frames_initially_gifted_to_the_linux_kernel
		{
			for frame_number in FillQueue::first_frame_not_initially_gifted_to_the_linux_kernel(number_of_frames_initially_gifted_to_the_linux_kernel) .. AlignedFrameNumber(number_of_frames)
			{
				self.push(frame_number)
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn push(&self, frame_number: AlignedFrameNumber)
	{
		debug_asert!(frame_number < FrameNumber(self.capacity() as u32));
		
		self.0.push(frame_number).expect("Pushed the same frame_number more than once!")
	}
	
	#[inline(always)]
	pub(crate) fn pop(&self) -> Option<AlignedFrameNumber>
	{
		self.0.pop().ok()
	}
}
