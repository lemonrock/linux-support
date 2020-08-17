// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Bit set aware.
#[macro_export]
macro_rules! bit_set_aware
{
	($type: ty) =>
	{
		impl std::convert::TryFrom<u16> for $type
		{
			type Error = BitSetAwareTryFromU16Error;

			#[inline(always)]
			fn try_from(value: u16) -> Result<Self, Self::Error>
			{
				if unlikely!(value < Self::OneBasedCorrection)
				{
					Err(BitSetAwareTryFromU16Error::default())
				}
				else if unlikely!(value >= Self::LinuxMaximum)
				{
					Err(BitSetAwareTryFromU16Error::default())
				}
				else
				{
					Ok(Self::hydrate(value))
				}
			}
		}

		impl std::convert::TryFrom<usize> for $type
		{
			type Error = BitSetAwareTryFromU16Error;

			#[inline(always)]
			fn try_from(value: usize) -> Result<Self, Self::Error>
			{
				if unlikely!(value < (Self::OneBasedCorrection as usize))
				{
					Err(BitSetAwareTryFromU16Error::default())
				}
				else if unlikely!(value >= (Self::LinuxMaximum as usize))
				{
					Err(BitSetAwareTryFromU16Error::default())
				}
				else
				{
					Ok(Self::hydrate(value as u16))
				}
			}
		}

		impl $crate::strings::parse_number::ParseNumber for $type
		{
			#[inline(always)]
			fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
			{
				let value = u16::parse_number(bytes, radix, parse_byte)?;
				if unlikely!(value < Self::OneBasedCorrection)
				{
					Err(ParseNumberError::TooSmall)
				}
				else if unlikely!(value >= Self::LinuxMaximum)
				{
					Err(ParseNumberError::TooLarge)
				}
				else
				{
					Ok(Self::hydrate(value))
				}
			}
		}

		impl Into<u32> for $type
		{
			#[inline(always)]
			fn into(self) -> u32
			{
				let value: u16 = self.into();
				value as u32
			}
		}

		impl Into<u64> for $type
		{
			#[inline(always)]
			fn into(self) -> u64
			{
				let value: u16 = self.into();
				value as u64
			}
		}

		impl Into<usize> for $type
		{
			#[inline(always)]
			fn into(self) -> usize
			{
				let value: u16 = self.into();
				value as usize
			}
		}

		impl Into<i32> for $type
		{
			#[inline(always)]
			fn into(self) -> i32
			{
				let value: u32 = self.into();
				value as i32
			}
		}

		impl Into<i64> for $type
		{
			#[inline(always)]
			fn into(self) -> i64
			{
				let value: u64 = self.into();
				value as i64
			}
		}

		impl Into<isize> for $type
		{
			#[inline(always)]
			fn into(self) -> isize
			{
				let value: usize = self.into();
				value as isize
			}
		}
	}
}
