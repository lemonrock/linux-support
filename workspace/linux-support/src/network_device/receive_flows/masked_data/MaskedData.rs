// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Masked data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields, bound(deserialize = "U: DeserializeOwned"))]
pub struct MaskedData<U: Unmasked>
{
	/// Data.
	pub data: U,
	
	/// This is the inverse of the data being masked.
	///
	/// If all bits are zero then the data is not used.
	///
	/// If all bits are one then an exact match is made.
	pub mask: Masked<U>,
}

impl<U: Unmasked> Default for MaskedData<U>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			data: U::default(),
			
			mask: Masked::Unused,
		}
	}
}

impl<U: Unmasked> MaskedData<U>
{
	#[inline(always)]
	pub(crate) fn from_underlying_data_and_mask(underlying_data: U::Underlying, underlying_mask: U::Underlying) -> Self
	{
		Self
		{
			data: U::from_underlying(underlying_data),
			
			mask: Masked::from_underlying(underlying_mask)
		}
	}
	
	#[inline(always)]
	pub(crate) fn underlying_data(&self) -> U::Underlying
	{
		self.data.underlying()
	}
	
	#[inline(always)]
	pub(crate) fn underlying_mask(&self) -> U::Underlying
	{
		self.mask.underlying()
	}
}
