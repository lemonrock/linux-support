// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cache timestamp.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CacheTimestampInHundrethsOfSeconds(u32);

impl Into<SystemTime> for CacheTimestampInHundrethsOfSeconds
{
	#[inline(always)]
	fn into(self) -> SystemTime
	{
		let duration = Duration::from_millis((self.0 as u64) * 10);
		
		SystemTime::UNIX_EPOCH + duration
	}
}
