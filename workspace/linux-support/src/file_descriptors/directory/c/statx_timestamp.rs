// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct statx_timestamp
{
	pub(super) tv_sec: i64,
	pub(super) tv_nsec: u32,
	__statx_timestamp_pad1: i32,
}

impl statx_timestamp
{
	#[inline(always)]
	fn zero_padding(&mut self)
	{
		self.__statx_timestamp_pad1 = 0;
	}
}
