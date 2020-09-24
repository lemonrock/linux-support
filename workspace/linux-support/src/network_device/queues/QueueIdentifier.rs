// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Queue identifier.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueueIdentifier(pub u16);

bit_set_aware!(QueueIdentifier);

impl Into<u16> for QueueIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl BitSetAware for QueueIdentifier
{
	/// This is a guess.
	const LinuxMaximum: u32 = 65536;

	const InclusiveMinimum: Self = Self(0);

	const InclusiveMaximum: Self = Self((Self::LinuxMaximum - 1) as u16);

	const Prefix: &'static [u8] = b"";

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!((value as u32) < Self::LinuxMaximum);

		Self(value)
	}
}

impl ParseNumber for QueueIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		use self::ParseNumberError::*;
		
		u16::parse_number(bytes, radix, parse_byte).map(Self)
	}
}

impl TryFrom<ExpressDataPathQueueIdentifier> for QueueIdentifier
{
	type Error = TryFromIntError;
	
	#[inline(always)]
	fn try_from(value: ExpressDataPathQueueIdentifier) -> Result<Self, Self::Error>
	{
		value.0.try_into().map(Self)
	}
}

impl QueueIdentifier
{
	/// Minimum.
	pub const InclusiveMinimum: Self = QueueCount::InclusiveMinimum.to_queue_identifier();
	
	/// Maximum.
	pub const InclusiveMaximum: Self = QueueCount::InclusiveMaximum.to_queue_identifier();
}
