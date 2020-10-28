// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A 31 bit unsigned integer that specifies a time in seconds.
#[derive(Default, Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct TimeInSeconds(BigEndianI32);

impl PartialOrd for TimeInSeconds
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		Some(self.cmp(rhs))
	}
}

impl Ord for TimeInSeconds
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		let left: u32 = (*self).into();
		let right: u32 = (*rhs).into();
		left.cmp(&right)
	}
}

impl PartialEq for TimeInSeconds
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		let left: u32 = (*self).into();
		let right: u32 = (*rhs).into();
		left.eq(&right)
	}
}

impl Eq for TimeInSeconds
{
}

impl Hash for TimeInSeconds
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		let seconds: U31SecondsDuration = (*self).into();
		seconds.hash(state)
	}
}

impl Into<u32> for TimeInSeconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let into: U31SecondsDuration = self.into();
		into.into()
	}
}

impl Into<U31SecondsDuration> for TimeInSeconds
{
	#[inline(always)]
	fn into(self) -> U31SecondsDuration
	{
		// RFC 2181, Section 8, paragraph 2: "Implementations should treat TTL values received with the most significant bit set as if the entire value received was zero".
		let top_byte = unsafe { *(self.0).get_unchecked(0) };
		if likely!(top_byte & 0x80 == 0)
		{
			U31SecondsDuration::try_from(u32::from_be_bytes(self.0)).unwrap()
		}
		else
		{
			U31SecondsDuration::Zero
		}
	}
}

impl TimeInSeconds
{
	/// Cache until?
	///
	/// Returns `None` if should not be cached.
	#[inline(always)]
	pub fn cache_until(self, now: NanosecondsSinceUnixEpoch) -> Option<NanosecondsSinceUnixEpoch>
	{
		let u31_seconds_duration: U31SecondsDuration = self.into();
		if unlikely!(u31_seconds_duration.is_zero())
		{
			None
		}
		else
		{
			Some(now + u31_seconds_duration)
		}
	}
}
