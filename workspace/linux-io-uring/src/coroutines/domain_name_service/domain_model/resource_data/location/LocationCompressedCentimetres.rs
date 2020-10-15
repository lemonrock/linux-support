// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Location centimetres.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationCompressedCentimetres(u8);

impl LocationCompressedCentimetres
{
	/// For example, the value 0x12 means 1 * 10^2 or 100cm.
	/// 0x99 means 9 * 10^9 or 90,000,000m.
	#[inline(always)]
	pub fn to_centimetres(self) -> u64
	{
		let scalar = (self.0 >> 4) as u64;

		let power_of_ten = (self.0 & 0b0000_1111) as u32;

		scalar * 10u64.pow(power_of_ten)
	}
}
