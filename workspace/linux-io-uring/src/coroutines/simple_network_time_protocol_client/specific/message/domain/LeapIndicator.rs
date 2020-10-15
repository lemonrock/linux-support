// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Leap Indicator (LI).
///
/// 2 bits in size (0 to 3 inclusive; all values defined).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum LeapIndicator
{
	NoWarning = 0,
	
	LastMinuteHas61Seconds = 1,
	
	LastMinuteHas59Seconds = 2,
	
	AlarmConditionClockNotSynchronized = 3,
}
