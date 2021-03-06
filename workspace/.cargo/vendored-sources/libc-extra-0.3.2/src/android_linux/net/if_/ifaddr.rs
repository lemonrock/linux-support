// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[allow(missing_debug_implementations)]
#[allow(missing_copy_implementations)]
pub struct ifaddr
{
	pub ifa_addr: sockaddr,
	pub ifa_ifu: Union_Unnamed1,
	pub ifa_ifp: *mut iface,
	pub ifa_next: *mut ifaddr,
}

impl Default for ifaddr
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union_Unnamed1
{
	pub _bindgen_data_: [u16; 8],
}

impl Union_Unnamed1
{
	pub unsafe fn ifu_broadaddr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifu_dstaddr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
}

impl Default for Union_Unnamed1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
