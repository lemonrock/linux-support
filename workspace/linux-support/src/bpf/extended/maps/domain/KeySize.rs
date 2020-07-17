// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Key size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct KeySize(NonZeroU16);

impl TryFrom<usize> for KeySize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: usize) -> Result<Self, Self::Error>
	{
		if unlikely!(value > u16::MAX as usize)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Self::try_from(value as u16)
		}
	}
}

impl TryFrom<u16> for KeySize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			let non_zero = unsafe { NonZeroU16::new_unchecked(value) };
			Self::try_from(non_zero)
		}
	}
}

impl TryFrom<NonZeroU16> for KeySize
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if value > Self::InclusiveMaximum.0
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl KeySize
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self::new_unsafe(1);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self::new_unsafe(MAX_BPF_STACK as u16);
	
	#[inline(always)]
	const fn new_unsafe(value: u16) -> Self
	{
		Self(unsafe { NonZeroU16::new_unchecked(value) })
	}
	
	#[inline(always)]
	pub(crate) const fn to_non_zero_u32(self) -> NonZeroU32
	{
		unsafe { NonZeroU32::new_unchecked(self.0.get() as u32) }
	}
	
	#[inline(always)]
	pub(crate) fn try_from_key_size<K: Sized>() -> Result<Self, ParseNumberError>
	{
		Self::try_from(size_of::<K>())
	}
}
