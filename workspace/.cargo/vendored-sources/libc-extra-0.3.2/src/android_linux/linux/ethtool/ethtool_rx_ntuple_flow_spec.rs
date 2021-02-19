// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct ethtool_rx_ntuple_flow_spec
{
	pub flow_type: u32,
	pub h_u: Union_Unnamed1,
	pub m_u: Union_Unnamed1,
	pub vlan_tag: u16,
	pub vlan_tag_mask: u16,
	pub data: u64,
	pub data_mask: u64,
	pub action: i32,
}

impl Clone for ethtool_rx_ntuple_flow_spec
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ethtool_rx_ntuple_flow_spec
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

#[repr(C)]
#[derive(Copy)]
#[allow(missing_debug_implementations)]
pub struct Union_Unnamed1
{
	pub _bindgen_data_: [u32; 18],
}

impl Union_Unnamed1
{
	pub unsafe fn tcp_ip4_spec(&mut self) -> *mut ethtool_tcpip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn udp_ip4_spec(&mut self) -> *mut ethtool_tcpip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn sctp_ip4_spec(&mut self) -> *mut ethtool_tcpip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ah_ip4_spec(&mut self) -> *mut ethtool_ah_espip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn esp_ip4_spec(&mut self) -> *mut ethtool_ah_espip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn usr_ip4_spec(&mut self) -> *mut ethtool_usrip4_spec
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn ether_spec(&mut self) -> *mut ethhdr
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
	
	pub unsafe fn hdata(&mut self) -> *mut [u8; 72]
	{
		let raw: *mut u8 = transmute(&self._bindgen_data_);
		transmute(raw.offset(0))
	}
}

impl Clone for Union_Unnamed1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for Union_Unnamed1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
