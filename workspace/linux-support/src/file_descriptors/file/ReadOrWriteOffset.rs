// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read or write offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadOrWriteOffset
{
	/// Read or write at a fixed offset in the file.
	///
	/// Can not exceed `i64::MAX as u64` otherwise bad things can happen; this is checked for in debug builds which will panic.
	Fixed(u64),

	/// Read or write at the current offset in the file.
	///
	/// It'll use (and update) the current file position.
	/// This obviously comes with the caveat that if the application has multiple reads or writes in flight, then the end result will not be as expected.
	/// This is similar to threads sharing a file descriptor and doing IO using the current file position.
	///
	/// Only available if the `ParametersFeaturesFlag::ReadWriteCurrentPosition` is set (which is true for Linux 5.6).
	CurrentFilePosition,
}

impl Into<i64> for ReadOrWriteOffset
{
	#[inline(always)]
	fn into(self) -> i64
	{
		use self::ReadOrWriteOffset::*;

		match self
		{
			Fixed(value) =>
				{
					debug_assert!(value <= i32::MAX as u64);
					value as i64
				}

			CurrentFilePosition => -1,
		}
	}
}
impl Into<u64> for ReadOrWriteOffset
{
	#[inline(always)]
	fn into(self) -> u64
	{
		let value: i64 = self.into();
		unsafe { transmute(value) }
	}
}
