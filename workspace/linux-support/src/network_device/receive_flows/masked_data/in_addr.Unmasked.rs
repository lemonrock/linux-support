// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl Unmasked for in_addr
{
	const UnderlyingZero: Self::Underlying = [0; 4];
	
	type Underlying = BigEndianU32;
	
	#[inline(always)]
	fn into_mask(self) -> Masked<Self>
	{
		let mut underlying: BigEndianU32 = unsafe { transmute(self.s_addr) };
		invert_bytes(&mut underlying);
		Masked::from_underlying(underlying)
	}
	
	#[inline(always)]
	fn from_underlying(underlying: Self::Underlying) -> Self
	{
		Self
		{
			s_addr: unsafe { transmute(underlying) }
		}
	}
	
	#[inline(always)]
	fn from_underlying_inverted(mut underlying_inverted: Self::Underlying) -> Self
	{
		invert_bytes(&mut underlying_inverted);
		Self
		{
			s_addr: unsafe { transmute(underlying_inverted) }
		}
	}
	
	#[inline(always)]
	fn underlying(&self) -> Self::Underlying
	{
		unsafe { transmute(self.s_addr) }
	}
}
