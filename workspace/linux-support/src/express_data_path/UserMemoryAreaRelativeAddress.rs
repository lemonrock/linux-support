// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A pointer recorded as a relative offset from the base of the `UserMemoryArea`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UserMemoryAreaRelativeAddress(u64);

impl TryInto<NonZeroU64> for UserMemoryAreaRelativeAddress
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_into(self) -> Result<NonZeroU64, Self::Error>
	{
		let value = self.0;
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(unsafe { NonZeroU64::new_unchecked(value) })
		}
	}
}

impl From<u64> for UserMemoryAreaRelativeAddress
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self(value)
	}
}

impl Into<u64> for UserMemoryAreaRelativeAddress
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Add<usize> for UserMemoryAreaRelativeAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, offset: usize) -> Self::Output
	{
		Self(self.0 + (offset as u64))
	}
}

impl Sub<Self> for UserMemoryAreaRelativeAddress
{
	type Output = usize;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		debug_assert!(self >= rhs);
		
		let difference = self.0 - rhs.0;
		debug_assert!(difference <= (usize::MAX as u64));
		
		Self(difference)
	}
}

impl Sub<usize> for UserMemoryAreaRelativeAddress
{
	type Output = Self;
	
	#[inline(always)]
	fn sub(self, rhs: usize) -> Self::Output
	{
		let rhs = rhs as u64;
		debug_assert!(self.0 >= rhs);
		
		Self(self.0 - rhs)
	}
}

impl UserMemoryAreaRelativeAddress
{
	#[inline(always)]
	const fn from_u64(value: u64) -> Self
	{
		Self(value)
	}
	
	#[inline(always)]
	const fn into_u64(self) -> u64
	{
		self.0
	}
}
