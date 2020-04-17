// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Whilst this is present in libc, it does not support useful derives.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct sockaddr_in
{
	/// Socket address family.
	sin_family: sa_family_t,

	/// Must a 16-bit integer in Network Endian form, not Native Endian form.
	pub sin_port: in_port_t,

	/// Address.
	pub sin_addr: in_addr,

	/// Must always be zero.
	pub sin_zero: [u8; 8],
}

impl Default for sockaddr_in
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			sin_family: AF_INET as sa_family_t,
			sin_port: 0,
			sin_addr: Default::default(),
			sin_zero: unsafe { zeroed() },
		}
	}
}

impl AsRef<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn as_ref(&self) -> &SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl AsMut<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl Borrow<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn borrow(&self) -> &SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl BorrowMut<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl Deref for sockaddr_in
{
	type Target = SocketAddrV4;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl DerefMut for sockaddr_in
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { transmute(self) }
	}
}

impl From<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn from(value: SocketAddrV4) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl Into<SocketAddrV4> for sockaddr_in
{
	#[inline(always)]
	fn into(self) -> SocketAddrV4
	{
		unsafe { transmute(self) }
	}
}

impl SocketData for sockaddr_in
{
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		self.sin_family
	}
}
