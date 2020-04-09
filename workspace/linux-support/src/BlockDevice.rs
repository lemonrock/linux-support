// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A block device.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockDevice
{
	/// Major.
	pub major: u8,

	/// Minor.
	pub minor: u8,
}

impl From<(u8, u8)> for BlockDevice
{
	#[inline(always)]
	fn from(value: (u8, u8)) -> Self
	{
		Self
		{
			major: value.0,
			minor: value.1,
		}
	}
}

impl BlockDevice
{
	const ZeroZero: Self = Self
	{
		major: 0,
		minor: 0,
	};

	#[inline(always)]
	pub(crate) fn is_not_zero_zero(self) -> bool
	{
		self != Self::ZeroZero
	}
}
