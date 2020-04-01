// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Interpretation of time values when setting timer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum TimerSetChoices
{
	/// Time values are a relative offset (duration).
	///
	/// This is the default.
	Relative = 0,

	/// Time values are an absolute.
	Absolute = TFD_TIMER_ABSTIME,

	/// Cancels reads if the realtime clock is adjusted.
	///
	/// Only works for `TimerClock::RealTime` or `TimerClock::RealTimeAlarm`.
	AbsoluteCancellingReadsOnFileDescriptorIfRealTimeClockIsAdjusted = TFD_TIMER_ABSTIME | TFD_TIMER_CANCEL_ON_SET,
}

impl Default for TimerSetChoices
{
	#[inline(always)]
	fn default() -> Self
	{
		TimerSetChoices::Relative
	}
}
