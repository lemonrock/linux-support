// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct in_addr
{
	/// Must a 32-bit integer in Network Endian form, not Native Endian form.
	pub s_addr: in_addr_t,
}

impl Into<Ipv4Addr> for in_addr
{
	#[inline(always)]
	fn into(self) -> Ipv4Addr
	{
		unsafe { transmute(self) }
	}
}

impl Into<IpAddr> for in_addr
{
	#[inline(always)]
	fn into(self) -> IpAddr
	{
		IpAddr::V4(self.into())
	}
}
