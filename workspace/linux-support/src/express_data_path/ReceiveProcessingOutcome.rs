// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive processing outcome.
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
	///
	/// Can also be used to implement forwarding of frames.
	RetainedFramePickAnotherOneFromUnusedFrames,
}
