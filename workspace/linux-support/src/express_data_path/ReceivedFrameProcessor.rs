// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveProcessingOutcome
{
	/// Also known as `drop`.
	GiftFrameBackToKernelForAnotherReceive,

	/// This will starve the Linux kernel of frames.
	ReturnFrameToUnusedFrames,

	/// This keeps the Linux kernel topped-up unless all available frames have been exhausted.
	///
	/// Needed for IP defragmentation.
	RetainedFramePickAnotherOneFromUnusedFrames,

	// TODO
	Forward,
}

/// Processes received frames (or no received frames).
pub trait ReceivedFrameProcessor
{
	/// For receive-only, normally a `bool`, where `true` implies the frame can be gifted back to the kernel.
	///
	/// For forwarding, always `()` as the frame is always gifted back to the kernel.
	type ProcessingOutcome;
	
	/// Nothing received; by default, calls `self.begin(0)` then `self.end()`.
	#[inline(always)]
	fn nothing_received(&mut self)
	{
		self.begin(0);
		self.end()
	}
	
	/// Called before `begin()`.
	///
	/// Defaults to `16`.
	///
	/// Other sensible values are `32` and `64`.
	///
	/// Effectively, a batch size.
	#[inline(always)]
	fn maximum_number_of_frames(&self) -> NonZeroU32
	{
		unsafe { NonZeroU32::new_unchecked(16) }
	}
	
	/// `received_number_of_frames` will always be less than or equal to `self.maximum_number_of_frames()`.
	fn begin(&mut self, received_number_of_frames: u32);
	
	/// Called as many times as the value of `received_number_of_frames` in `begin()`.
	///
	/// `relative_frame_index` starts at `0` and is incremented by `1` for each call after `begin()`.
	///
	/// Once this method returns, the memory of the `received_frame` will be re-used.
	///
	/// `relative_frame_index` is always less than `received_number_of_frames` in `self.begin()`.
	///
	/// After `end()` is called `relative_frame_index` may be re-used.
	fn process_received_frame<'a>(&mut self, relative_frame_index: u32, received_frame: &'a mut [u8]) -> Self::Outcome;
	
	/// If we retained the frame in `process_received_frame()` above yet there are no unused frames - hence we potentially starve the Linux kernel.
	#[inline(always)]
	fn no_more_unused_frames_to_gift_to_linux_kernel(&mut self)
	{
	}
	
	/// Finished.
	fn end(&mut self)
	{
	}
}
