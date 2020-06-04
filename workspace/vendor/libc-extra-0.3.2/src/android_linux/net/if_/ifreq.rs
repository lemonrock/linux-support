// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ifreq
{
	pub ifr_ifrn: Union_Unnamed2,
	pub ifr_ifru: Union_Unnamed3,
}

impl Default for ifreq
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union_Unnamed2
{
	pub _bindgen_data_: [u8; IF_NAMESIZE],
}

impl Union_Unnamed2
{
	pub unsafe fn ifrn_name(&mut self) -> *mut [c_char; IF_NAMESIZE]
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
}

impl Default for Union_Unnamed2
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Union_Unnamed3
{
	pub _bindgen_data_: [u64; 3],
}

impl Union_Unnamed3
{
	pub unsafe fn ifru_addr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_dstaddr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_broadaddr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_netmask(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_hwaddr(&mut self) -> *mut sockaddr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_flags(&mut self) -> *mut c_short
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_ivalue(&mut self) -> *mut c_int
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_mtu(&mut self) -> *mut c_int
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_map(&mut self) -> *mut ifmap
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_slave(&mut self)  -> *mut [c_char; IF_NAMESIZE]
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_newname(&mut self) -> *mut [c_char; IF_NAMESIZE]
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ifru_data(&mut self) -> *mut *mut c_void
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
}

impl Default for Union_Unnamed3
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
