// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// IPsec Security Parameter Index (SPI).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct SecurityParameterIndex(BigEndianU32);

impl Unmasked for SecurityParameterIndex
{
	const UnderlyingZero: Self::Underlying = [0; 4];
	
	type Underlying = BigEndianU32;
	
	#[inline(always)]
	fn into_mask(self) -> Masked<Self>
	{
		let mut underlying = self.0;
		invert_bytes(&mut underlying);
		Masked::from_underlying(underlying)
	}
	
	#[inline(always)]
	fn from_underlying(underlying: Self::Underlying) -> Self
	{
		Self(underlying)
	}
	
	#[inline(always)]
	fn from_underlying_inverted(mut underlying_inverted: Self::Underlying) -> Self
	{
		invert_bytes(&mut underlying_inverted);
		Self(underlying_inverted)
	}
	
	#[inline(always)]
	fn underlying(&self) -> Self::Underlying
	{
		self.0
	}
}
