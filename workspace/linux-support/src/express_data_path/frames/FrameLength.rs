// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Length of an Ethernet frame in bytes.
///
/// Minimum is 64 bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameLength(NonZeroU16);

impl TryFrom<u8> for FrameLength
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		Self::try_from(value as u16)
	}
}

impl TryFrom<u16> for FrameLength
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Self::try_from(unsafe { NonZeroU16::new_unchecked(value) })
		}
	}
}

impl TryFrom<NonZeroU16> for FrameLength
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU16) -> Result<Self, Self::Error>
	{
		if unlikely!(value < Self::InclusiveMinimum.0)
		{
			Err(ParseNumberError::TooSmall)
		}
		else if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooSmall)
		}
		else
		{
			Self(value)
		}
	}
}

impl Into<NonZeroU16> for FrameLength
{
	#[inline(always)]
	fn into(self) -> NonZeroU16
	{
		self.0
	}
}

impl Into<u16> for FrameLength
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0.get()
	}
}

impl Into<u32> for FrameLength
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0.get() as u32
	}
}

impl Into<usize> for FrameLength
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0.get() as usize
	}
}

impl FrameLength
{
	/// This is 64 bytes.
	pub const InclusiveMinimum: Self = Self::new_unchecked(64);
	
	/// This is the upper limit of `FrameSize`.
	pub const InclusiveMaximum: Self = Self::new_unchecked(ChunkSize::_4096 as u16);
	
	#[inline(always)]
	const fn new_unchecked(value: u16) -> Self
	{
		Self(unsafe { NonZeroU16::new_unchecked(value) })
	}
}
