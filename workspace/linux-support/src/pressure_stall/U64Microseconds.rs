// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Microseconds (u64).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct U64Microseconds(pub u64);

impl ParseNumber for U64Microseconds
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(u64::parse_number(bytes, radix, parse_byte)?))
	}
}
