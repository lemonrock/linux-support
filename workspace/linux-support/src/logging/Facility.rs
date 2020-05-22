// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 5424, Section 6.2.1: "Facility values MUST be in the range of 0 to 23 inclusive".
///
/// However, facilities 24 to 31 inclusive are not permissible but have historically been used, eg on Mac OS X.
#[derive(Copy, Clone)]
#[repr(C)]
pub union Facility
{
	known: KnownFacility,
	unknown: UnknownFacility,
}

impl TryInto<KnownFacility> for Facility
{
	type Error = UnknownFacility;
	
	#[inline(always)]
	fn try_into(self) -> Result<KnownFacility, Self::Error>
	{
		if likely!(self.raw_value_is_known())
		{
			Ok(unsafe { self.known })
		}
		else
		{
			Err(unsafe { self.unknown })
		}
	}
}

impl Facility
{
	#[inline(always)]
	const fn raw_value_is_known(self) -> bool
	{
		self.raw_value() <= (KnownFacility::local_use_7 as u8)
	}
	
	#[inline(always)]
	const fn raw_value(self) -> u8
	{
		unsafe { transmute(self.known) }
	}
}
