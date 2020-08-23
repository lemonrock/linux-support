// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Revision.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Revision(u8);

impl From<u8> for Revision
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		Self(value)
	}
}

impl Into<u8> for Revision
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl FromBytes for Revision
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		Ok(Self(u8::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(bytes, size_of::<u8>() * 2)?))
	}
}
