// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Very similar to `SystemTime` but without any concerns about time running backwards.
///
/// Internally modelled as a `Duration` offset from the Unix Epoch.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NanosecondsSinceUnixEpoch(Duration);

impl Add<U31SecondsDuration> for NanosecondsSinceUnixEpoch
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, lhs: U31SecondsDuration) -> Self::Output
	{
		Self(self.0.checked_add(lhs.into_rust_duration()).unwrap())
	}
}

impl NanosecondsSinceUnixEpoch
{
	/// Now.
	#[inline(always)]
	pub fn now() -> Self
	{
		Self(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap())
	}
	
	/// From seconds (as an `u64`).
	#[inline(always)]
	pub const fn from_seconds_u64(seconds: u64) -> Self
	{
		Self(Duration::from_secs(seconds))
	}
	
	/// From seconds (as an `u32`).
	#[inline(always)]
	pub const fn from_seconds_u32(seconds: u32) -> NanosecondsSinceUnixEpoch
	{
		Self::from_seconds_u64(seconds as u64)
	}
	
	const U32PeriodLengthInSeconds: u64 = u32::MAX as u64;
	
	// Increment this to 1 after Sunday, February 7, 2106 6:28:15 AM GMT.
	const ElapsedU32WrapAroundPoints: u64 = 0;
	
	const LastU32WrapAroundPoint: u64 = Self::U32PeriodLengthInSeconds * Self::ElapsedU32WrapAroundPoints;
	
	const NextU32WrapAroundPoint: u64 = Self::LastU32WrapAroundPoint + Self::U32PeriodLengthInSeconds;
	
	/// From seconds (as an `u32`), taking into account wrap-around due to the small size of an u32.
	#[inline(always)]
	pub const fn from_seconds_u32_before_next_wrap_around(seconds: u32) -> NanosecondsSinceUnixEpoch
	{
		Self::from_seconds_u64(Self::LastU32WrapAroundPoint + (seconds as u64))
	}
	
	/// From seconds (as an `u32`), taking into account wrap-around that has happened due to the small size of an u32.
	#[inline(always)]
	pub const fn from_seconds_u32_after_next_wrap_around(seconds: u32) -> NanosecondsSinceUnixEpoch
	{
		Self::from_seconds_u64(Self::NextU32WrapAroundPoint + (seconds as u64))
	}
	
}
