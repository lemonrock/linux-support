// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defaults to `Void`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BtfTypeIdentifier(u32);

impl Into<u32> for BtfTypeIdentifier
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl BtfTypeIdentifier
{
	/// `void`.
	pub const Void: Self = Self(0);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(BTF_MAX_TYPE as u32);
	
	/// New instance.
	#[inline(always)]
	pub const fn new(value: NonZeroU32) -> Self
	{
		Self(value.get())
	}
	
	#[inline(always)]
	pub(crate) fn next(&mut self) -> Result<Self, BtfTypeError>
	{
		if unlikely!(self == &mut Self::InclusiveMaximum)
		{
			return Err(BtfTypeError::TooManyBtfTypes)
		}
		
		self.0 += 1;
		Ok(*self)
	}
}
