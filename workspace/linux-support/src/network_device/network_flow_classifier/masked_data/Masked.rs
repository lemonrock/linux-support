// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Masked.
///
/// This is the invert of the value, eg, for the IPv4 mask 255.0.0.0, field `0` is 0.255.255.255.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Masked<U: Unmasked>(U::Underlying, PhantomData<U>);

impl<T: Unmasked> Default for Masked<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Unused
	}
}

impl<U: Unmasked> Masked<U>
{
	/// Unused.
	pub const Unused: Self = Self::from_underlying(U::UnderlyingZero);
	
	/// New instance.
	#[inline(always)]
	pub const fn from_underlying(underlying: U::Underlying) -> Self
	{
		Self(underlying, PhantomData)
	}
	
	/// Is this item masked such that it is not used?
	#[inline(always)]
	pub fn is_zero_and_so_item_is_unused(self) -> bool
	{
		self == Self::Unused
	}
	
	#[allow(dead_code)]
	#[inline(always)]
	fn into_unmasked(self) -> U
	{
		U::from_underlying_inverted(self.0)
	}
	
	#[inline(always)]
	fn underlying(&self) -> U::Underlying
	{
		self.0
	}
}
