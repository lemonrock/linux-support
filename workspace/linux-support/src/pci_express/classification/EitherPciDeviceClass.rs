// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// New type to permit implementation of `FromBytes`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct EitherPciDeviceClass(pub Either<PciDeviceClass, (u8, u8, u8)>);

impl FromBytes for EitherPciDeviceClass
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let u24 = u32::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(bytes, 6)?;
		Ok(Self(PciDeviceClass::parse(u24)))
	}
}
