// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive low water mark.
///
/// `recv()`, `recvmsg()` and `read()` for stream (ie TCP) sockets will not return data until this low water mark is met.
///
/// The default is 1 (one byte).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ReceiveLowWaterMarkInBytes(NonZeroU32);

impl TryFrom<NonZeroU32> for ReceiveLowWaterMarkInBytes
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Default for ReceiveLowWaterMarkInBytes
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl ReceiveLowWaterMarkInBytes
{
	/// Default.
	pub const Default: Self = Self(unsafe { NonZeroU32::new_unchecked(1) });
	
	/// Minimum.
	pub const InclusiveMinimum: Self = Self::Default;
	
	/// Maximum.
	pub const InclusiveMaximum: Self = Self(unsafe { NonZeroU32::new_unchecked(i32::MAX as u32) });
}
