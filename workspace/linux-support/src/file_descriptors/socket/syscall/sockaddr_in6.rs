// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sockaddr_in6
{
	/// Socket address family.
	sin6_family: sa_family_t,

	/// Must a 16-bit integer in Network Endian form, not Native Endian form.
	pub sin6_port: in_port_t,

	/// Must be a 32-bit integer in Network Endian form, not Native Endian form.
	pub sin6_flowinfo: u32,

	/// Must be a 128-bit integer in Network Endian form, not Native Endian form.
	pub sin6_addr: in6_addr,

	/// Must be a 32-bit integer in Network Endian form, not Native Endian form.
	pub sin6_scope_id: u32,
}

impl Default for sockaddr_in6
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sin6_family: AF_INET6 as sa_family_t,
			sin6_port: 0,
			sin6_flowinfo: 0,
			sin6_addr: unsafe { zeroed() },
			sin6_scope_id: 0,
		}
	}
}

impl AsRef<SocketAddrV6> for sockaddr_in6
{
	#[inline(always)]
	fn as_ref(&self) -> &SocketAddrV6
	{
		unsafe { transmute(self) }
	}
}

impl AsMut<SocketAddrV6> for sockaddr_in6
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut SocketAddrV6
	{
		unsafe { transmute(self) }
	}
}

impl Borrow<SocketAddrV6> for sockaddr_in
{
	#[inline(always)]
	fn borrow(&self) -> &SocketAddrV6
	{
		unsafe { transmute(self) }
	}
}

impl BorrowMut<SocketAddrV6> for sockaddr_in
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut SocketAddrV6
	{
		unsafe { transmute(self) }
	}
}

impl Deref for sockaddr_in6
{
	type Target = SocketAddrV6;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl DerefMut for sockaddr_in6
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl From<SocketAddrV6> for sockaddr_in6
{
	#[inline(always)]
	fn from(value: SocketAddrV6) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl Into<SocketAddrV6> for sockaddr_in6
{
	#[inline(always)]
	fn into(self) -> SocketAddrV6
	{
		unsafe { transmute(self) }
	}
}

impl SocketData for sockaddr_in6
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sin6_family
	}
}
