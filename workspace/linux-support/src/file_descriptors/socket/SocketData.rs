// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents socket data.
pub trait SocketData: Sized + Debug
{
	/// Address type, eg `in6_addr`.
	type Address: Sized;
	
	/// Socket family (eg `AF_UNIX`).
	fn family(&self) -> sa_family_t;
	
	/// Address.
	fn address(&self) -> &Self::Address;
	
	#[doc(hidden)]
	fn display_format(&self, f: &mut Formatter, address_length: usize) -> fmt::Result;

	#[doc(hidden)]
	#[inline(always)]
	fn specialized_drop(socket_file_descriptor: &mut SocketFileDescriptor<Self>)
	{
		socket_file_descriptor.0.close()
	}
}

impl SocketData for SocketAddrV4
{
	type Address = in_addr;
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.family()
	}
	
	#[inline(always)]
	fn address(&self) -> &Self::Address
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.address()
	}
	
	#[doc(hidden)]
	fn display_format(&self, f: &mut Formatter, address_length: usize) -> fmt::Result
	{
		let inner: &sockaddr_in = unsafe { transmute(self) };
		inner.display_format(f, address_length)
	}
}

impl SocketData for SocketAddrV6
{
	type Address = in6_addr;
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.family()
	}
	
	#[inline(always)]
	fn address(&self) -> &Self::Address
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.address()
	}
	
	#[doc(hidden)]
	fn display_format(&self, f: &mut Formatter, address_length: usize) -> fmt::Result
	{
		let inner: &sockaddr_in6 = unsafe { transmute(self) };
		inner.display_format(f, address_length)
	}
}
