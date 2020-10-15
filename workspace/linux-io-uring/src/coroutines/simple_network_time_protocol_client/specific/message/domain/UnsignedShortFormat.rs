// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct UnsignedShortFormat
{
	seconds: BigEndianU16,
	
	fraction: BigEndianU16,
}

impl Into<Unsigned1616FixedPoint> for SignedShortFormat
{
	#[inline(always)]
	fn into(self) -> Unsigned1616FixedPoint
	{
		Unsigned1616FixedPoint::from((self.seconds, self.fraction))
	}
}

impl UnsignedShortFormat
{
	pub(crate) const Zero: Self = Self
	{
		seconds: [0; 2],
		
		fraction: [0; 2],
	};
	
	#[inline(always)]
	pub(crate) fn is_zero(self) -> bool
	{
		self == Self::Zero
	}
}
