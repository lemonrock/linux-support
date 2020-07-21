// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strange representation of a listening socket used by BPF map.
///
/// Must not be implemented in downstream code.
pub trait SocketValue: Copy
{
	#[doc(hidden)]
	fn from_raw_fd(raw_fd: RawFd) -> Self;
}

impl SocketValue for u32
{
	#[inline(always)]
	fn from_raw_fd(raw_fd: RawFd) -> Self
	{
		raw_fd as Self
	}
}

impl SocketValue for u64
{
	#[inline(always)]
	fn from_raw_fd(raw_fd: RawFd) -> Self
	{
		raw_fd as Self
	}
}
