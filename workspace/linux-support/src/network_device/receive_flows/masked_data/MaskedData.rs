// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Masked data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MaskedData<T: Unmasked>
{
	/// Data.
	pub data: T,
	
	/// This is the inverse of the data being masked.
	///
	/// If zero then the data is not used.
	pub mask: Masked<T>,
}

impl<T: Unmasked> Default for MaskedData<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			data: T::default(),
			
			mask: Masked::Unused,
		}
	}
}

impl<T: Unmasked> MaskedData<T>
{
	#[inline(always)]
	pub(crate) fn from_underlying_data_and_mask(underlying_data: T::Underlying, underlying_mask: T::Underlying) -> Self
	{
		Self
		{
			data: T::from_underlying(underlying_data),
			
			mask: Masked::new(underlying_mask)
		}
	}
	
	#[inline(always)]
	pub(crate) fn underlying_data(&self) -> T::Underlying
	{
		self.data.underlying()
	}
	
	#[inline(always)]
	pub(crate) fn underlying_mask(&self) -> T::Underlying
	{
		self.mask.underlying()
	}
}
