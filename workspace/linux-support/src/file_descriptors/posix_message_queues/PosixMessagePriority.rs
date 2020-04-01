// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a message queue priority.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PosixMessagePriority(u16);

impl Into<u16> for PosixMessagePriority
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl Into<u32> for PosixMessagePriority
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl From<u8> for PosixMessagePriority
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		PosixMessagePriority(value as u16)
	}
}

impl TryFrom<u16> for PosixMessagePriority
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u16) -> Result<Self, Self::Error>
	{
		if value > 32767
		{
			Err(())
		}
		else
		{
			Ok(PosixMessagePriority(value))
		}
	}
}

impl TryFrom<u32> for PosixMessagePriority
{
	type Error = ();

	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if value > 32767
		{
			Err(())
		}
		else
		{
			Ok(PosixMessagePriority(value as u16))
		}
	}
}
