// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a one-based network interface, such that `eth0` might be `1`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct NetworkInterfaceIndex(NonZeroU32);

impl Into<NonZeroU32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		self.0
	}
}

impl Into<i32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn into(self) -> i32
	{
		let x: u32 = self.0.into();
		x as i32
	}
}

impl Into<NonZeroU64> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn into(self) -> NonZeroU64
	{
		self.0.into()
	}
}

impl From<NonZeroU8> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn from(value: NonZeroU8) -> Self
	{
		Self(value.into())
	}
}

impl From<NonZeroU16> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn from(value: NonZeroU16) -> Self
	{
		Self(value.into())
	}
}

impl From<NonZeroU32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn from(value: NonZeroU32) -> Self
	{
		Self(value)
	}
}

impl TryFrom<u8> for NetworkInterfaceIndex
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(())
		}
		else
		{
			Ok(Self(unsafe { NonZeroU32::new_unchecked(value as u32) }))
		}
	}
}

impl TryFrom<u16> for NetworkInterfaceIndex
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(())
		}
		else
		{
			Ok(Self(unsafe { NonZeroU32::new_unchecked(value as u32) }))
		}
	}
}

impl TryFrom<u32> for NetworkInterfaceIndex
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if value == 0
		{
			Err(())
		}
		else
		{
			Ok(Self(unsafe { NonZeroU32::new_unchecked(value) }))
		}
	}
}
