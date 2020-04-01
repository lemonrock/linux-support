// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Clocks ticks.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClockTicks(u64);

impl From<u64> for ClockTicks
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self(value)
	}
}

impl From<clock_t> for ClockTicks
{
	#[inline(always)]
	fn from(value: clock_t) -> Self
	{
		Self(value as u64)
	}
}

impl Into<u64> for ClockTicks
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}

impl Into<clock_t> for ClockTicks
{
	#[inline(always)]
	fn into(self) -> clock_t
	{
		self.0 as clock_t
	}
}

impl ParseNumber for ClockTicks
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(u64::parse_number(bytes, radix, parse_byte)?))
	}
}

impl ClockTicks
{
	/// New instance.
	#[inline(always)]
	pub const fn new(clock_ticks: u64) -> Self
	{
		Self(clock_ticks)
	}

	/// To seconds.
	#[inline(always)]
	pub fn to_seconds(self) -> u64
	{
		self.0 / Self::clock_ticks_per_second()
	}

	/// A value of `100` here means `100Hz`.
	///
	/// The number of times the Linux timer interrupts the CPU for scheduling and other tasks.
	///
	/// This value is always `100` in the musl libc.
	#[inline(always)]
	fn clock_ticks_per_second() -> u64
	{
		(unsafe { sysconf(_SC_CLK_TCK) }) as u64
	}
}
