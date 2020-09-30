// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl Unmasked for in6_addr
{
	const UnderlyingZero: Self::Underlying = [[0; 4]; 4];
	
	type Underlying = [BigEndianU32; 4];
	
	#[inline(always)]
	fn into_mask(self) -> Masked<Self>
	{
		let mut underlying = unsafe { self.s6_addr };
		invert_bytes(&mut underlying);
		Masked::new(unsafe { transmute(underlying) })
	}
	
	#[inline(always)]
	fn from_underlying(underlying: Self::Underlying) -> Self
	{
		Self
		{
			s6_addr32: underlying
		}
	}
	
	#[inline(always)]
	fn from_underlying_inverted(mut underlying_inverted: Self::Underlying) -> Self
	{
		let s6_addr: [u8; 16] = unsafe { transmute(underlying_inverted) };
		
		invert_bytes(&mut s6_addr);
		Self
		{
			s6_addr
		}
	}
	
	#[inline(always)]
	fn underlying(&self) -> Self::Underlying
	{
		self.s6_addr32
	}
}
