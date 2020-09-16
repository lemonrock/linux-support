// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Processes received frames (or no received frames).
pub trait ReceivedFrameProcessor
{
	/// Returns type from `self.end()`.
	type R: Sized;
	
	#[doc(hidden)]
	#[inline(always)]
	fn nothing_received(&mut self) -> Self::R
	{
		self.begin(0);
		self.end()
	}
	
	/// `received_number_of_frames` will always be less than `maximum_number_of_frames` in `ReceiveTransmitMemoryRingQueues::receive_and_drop()`.
	fn begin(&mut self, received_number_of_frames: u32);
	
	/// Once this method returns, the memory from the `received_frame` may have been re-used.
	///
	/// `relative_frame_index` is always less than `received_number_of_frames` in `self.begin()`.
	fn process_received_frame<'a>(&mut self, relative_frame_index: u32, received_frame: &'a [u8]);
	
	/// Finished.
	fn end(&mut self) -> Self::R;
}
