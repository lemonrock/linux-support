// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A relative amount representing the memory latency relative cost in accessing memory on a specific NUMA node relative to another.
///
/// This is found by dividing one NUMA node's MemoryLatencyRelativeCost by another.
///
/// Typical values are 10, 16 and 32 for AMD EPYC; for other systems, the values are often mis-reported by the BIOS to Linux (via the ACPI SLIT table) and are usually overestimates.
///
/// This is called 'distance' in Linux and in the ACPI SLIT table.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MemoryLatencyRelativeCost(NonZeroU8);

impl TryFrom<u8> for MemoryLatencyRelativeCost
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(new_non_zero_u8(value))
		}
	}
}

impl TryFrom<NonZeroU8> for MemoryLatencyRelativeCost
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		if unlikely!(value < Self::InclusiveMinimum.0)
		{
			Err(ParseNumberError::TooSmall)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Into<NonZeroU8> for MemoryLatencyRelativeCost
{
	#[inline(always)]
	fn into(self) -> NonZeroU8
	{
		self.0
	}
}

impl Into<u8> for MemoryLatencyRelativeCost
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0.get()
	}
}

impl ParseNumber for MemoryLatencyRelativeCost
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let raw_value = u8::parse_number(bytes, radix, parse_byte)?;
		Self::try_from(raw_value)
	}
}

impl MemoryLatencyRelativeCost
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(new_non_zero_u8(10));

	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(new_non_zero_u8(255));
}
