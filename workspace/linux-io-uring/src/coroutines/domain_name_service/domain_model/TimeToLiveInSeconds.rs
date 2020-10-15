// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A 31 bit unsigned integer that specifies the time interval (in seconds) that a resource record (RR) may be cached before it should be discarded.
///
/// See <https://tools.ietf.org/html/rfc2181#section-8>.
///
/// Zero values are interpreted to mean that the resource record can only be used for the transaction in progress, and should not be cached.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub struct TimeToLiveInSeconds(BigEndianU32);

impl From<BigEndianU32> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn from(seconds: BigEndianU32) -> Self
	{
		Self(seconds)
	}
}

impl Into<u32> for TimeToLiveInSeconds
{
	#[inline(always)]
	fn into(self) -> u32
	{
		let value = u32::from_be_bytes(self.0);

		// RFC 2181, Section 8; if the top bit is set, the value is zero.
		if unlikely!(value & 0x80000000 != 0)
		{
			0
		}
		else
		{
			value
		}
	}
}
