// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An offset value.
pub trait OffsetValue: Sized + Copy
{
	#[doc(hidden)]
	fn from_i128(value: i128) -> Result<Self, ()>;
	
	#[doc(hidden)]
	fn from_u128(value: u128) -> Result<Self, ()>;
	
	#[doc(hidden)]
	fn serialize<S: Serializer>(self, serializer: S) -> Result<S::Ok, S::Error>;
}

impl OffsetValue for i16
{
	#[inline(always)]
	fn from_i128(value: i128) -> Result<Self, ()>
	{
		if value >= (i16::MIN as i128) && value <= (i16::MAX as i128)
		{
			Ok(value as i16)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn from_u128(value: u128) -> Result<Self, ()>
	{
		if value <= (i16::MAX as u128)
		{
			Ok(value as i16)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn serialize<S: Serializer>(self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_i16(self)
	}
}

impl OffsetValue for i32
{
	#[inline(always)]
	fn from_i128(value: i128) -> Result<Self, ()>
	{
		if value >= (i32::MIN as i128) && value <= (i32::MAX as i128)
		{
			Ok(value as i32)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn from_u128(value: u128) -> Result<Self, ()>
	{
		if value <= (i32::MAX as u128)
		{
			Ok(value as i32)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn serialize<S: Serializer>(self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_i32(self)
	}
}

impl OffsetValue for u64
{
	#[inline(always)]
	fn from_i128(value: i128) -> Result<Self, ()>
	{
		if value >= (i64::MIN as i128) && value <= (i64::MAX as i128)
		{
			Ok(value as i64 as u64)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn from_u128(value: u128) -> Result<Self, ()>
	{
		if value <= (u64::MAX as u128)
		{
			Ok(value as u64)
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	fn serialize<S: Serializer>(self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_u64(self)
	}
}
