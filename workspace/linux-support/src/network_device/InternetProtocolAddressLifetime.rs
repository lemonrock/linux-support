// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Address lifetime.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct InternetProtocolAddressLifetime(pub(crate) u32);

impl TryFrom<u32> for InternetProtocolAddressLifetime
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value == Self::Infinite.0)
		{
			Err(ParseNumberError::WasMaximum)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl InternetProtocolAddressLifetime
{
	/// An infinite lifetime (``).
	pub const Infinite: Self = Self(0xFFFF_FFFF);
	
	/// Default temporary valid (`TEMP_VALID_LIFETIME`).
	pub const DefaultTemporaryValid: Self = Self(7 * 86_400);
	
	/// Default temporary preferred (`TEMP_PREFERRED_LIFETIME`).
	pub const DefaultTemporaryPreferred: Self = Self(86_400);
	
	/// `temporary_address_prefered_lifetime`.
	///
	/// Default is `TEMP_PREFERRED_LIFETIME` (`86,400`).
	/// Infinity is `INFINITY_LIFE_TIME` (`0xFFFF_FFFF`).
	
	/// Is infinite?
	#[inline(always)]
	pub const fn is_infinite(self) -> bool
	{
		self.0 == Self::Infinite.0
	}
	
	/// Into microseconds, or `None` if infinite.
	#[inline(always)]
	pub fn into_microseconds(self) -> Option<u32>
	{
		if self.is_infinite()
		{
			None
		}
		else
		{
			Some(self.0)
		}
	}
}
