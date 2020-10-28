// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A duration in seconds, with a maximum value of `(i32::MAX as u32)`.
///
/// Usage is discoraged except for compatibility with legacy code.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U31SecondsDuration(u32);

impl TryFrom<BigEndianU32> for U31SecondsDuration
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: BigEndianU32) -> Result<Self, Self::Error>
	{
		Self::try_from(u32::from_be_bytes(value))
	}
}

impl TryFrom<u32> for U31SecondsDuration
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if value > Self::InclusiveMaximum.0
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Into<u32> for U31SecondsDuration
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl Into<Duration> for U31SecondsDuration
{
	#[inline(always)]
	fn into(self) -> Duration
	{
		self.into_rust_duration()
	}
}

impl U31SecondsDuration
{
	/// Zero (same as InclusiveMinimum).
	pub const Zero: Self = Self(0);
	
	/// Inclusive maximum (same as Zero).
	pub const InclusiveMinimum: Self = Self::Zero;
	
	/// Inclusive minimum.
	pub const InclusiveMaximum: Self = Self(i32::MAX as u32);
	
	/// Into Rust `Duration` (as opposed to `chrono` crate's `Duration`).
	#[inline(always)]
	pub const fn into_rust_duration(self) -> Duration
	{
		Duration::from_secs(self.0 as u64)
	}
	
	/// Is zero.
	#[inline(always)]
	pub fn is_zero(self) -> bool
	{
		self == Self::Zero
	}
}
