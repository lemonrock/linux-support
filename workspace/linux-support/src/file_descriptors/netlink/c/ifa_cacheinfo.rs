// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents cache information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ifa_cacheinfo
{
	/// Can be `INFINITY_LIFE_TIME` (`0xFFFFFFFF` / `u32::MAX`).
	///
	/// Otherwise it is in microseconds.
	ifa_prefered: u32,
	
	/// Can be `INFINITY_LIFE_TIME` (`0xFFFFFFFF` / `u32::MAX`).
	///
	/// Otherwise it is in microseconds.
	ifa_valid: u32,
	
	/// Created timestamp in hundredths of seconds.
	cstamp: CacheTimestampInHundrethsOfSeconds,
	
	/// Updated timestamp in hundredths of seconds.
	tstamp: CacheTimestampInHundrethsOfSeconds,
}

impl ifa_cacheinfo
{
	pub(super) const INFINITY_LIFE_TIME: u32 = 0xFFFFFFFF;
	
	/// Preferred lifetime.
	#[inline(always)]
	pub fn preferred_life_time(&self) -> LifeTime
	{
		LifeTime::from(self.ifa_prefered)
	}
	
	/// Valid lifetime.
	#[inline(always)]
	pub fn valid_life_time(&self) -> LifeTime
	{
		LifeTime::from(self.ifa_valid)
	}
	
	/// Created.
	#[inline(always)]
	pub fn created(&self) -> CacheTimestampInHundrethsOfSeconds
	{
		self.cstamp
	}
	
	/// Updated.
	#[inline(always)]
	pub fn updated(&self) -> CacheTimestampInHundrethsOfSeconds
	{
		self.tstamp
	}
}
