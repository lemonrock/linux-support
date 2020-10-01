// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Four bytes are the start of an Internet Packet; could be TCP source and destination port numbers or IPSec security parameter index (SPI).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Layer4FourBytes(BigEndianU32);

impl From<SecurityParameterIndex> for Layer4FourBytes
{
	#[inline(always)]
	fn from(value: SecurityParameterIndex) -> Self
	{
		Self(value.0)
	}
}

impl From<(Layer4Port, Layer4Port)> for Layer4FourBytes
{
	#[inline(always)]
	fn from((source_port, destination_port): (Layer4Port, Layer4Port)) -> Self
	{
		let x =
		[
			source_port.0,
			destination_port.0
		];
		
		Self(unsafe { transmute(x) })
	}
}

impl Unmasked for Layer4FourBytes
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
