// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A time out for an EPoll wait.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPollTimeOut(i32);

impl From<u16> for EPollTimeOut
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		EPollTimeOut(value as i32)
	}
}

impl Into<i32> for EPollTimeOut
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self.0
	}
}

impl EPollTimeOut
{
	/// Never times out.
	pub const Infinite: Self = EPollTimeOut(-1);

	/// Always times outs.
	pub const Immediately: Self = EPollTimeOut(0);

	/// Times out in `n` milliseconds.
	#[inline(always)]
	pub const fn in_n_milliseconds(n_milliseconds: u16) -> Self
	{
		EPollTimeOut(n_milliseconds as i32)
	}
}
