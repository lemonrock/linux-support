// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ifreq_ifru
{
	/// `ifr_addr`.
	///
	/// Address.
	pub(crate) ifru_addr: sockaddr,
	
	/// `ifr_dstaddr`.
	///
	/// Other end of a point-to-point link.
	pub(crate) ifru_dstaddr: sockaddr,
	
	/// `ifr_broadaddr`.
	///
	/// Broadcast Address.
	pub(crate) ifru_broadaddr: sockaddr,
	
	/// `ifr_netmask`.
	///
	/// Interface netmask.
	pub(crate) ifru_netmask: sockaddr,
	
	/// `ifr_hwaddr`.
	///
	/// MAC address.
	pub(crate) ifru_hwaddr: sockaddr,
	
	/// `ifr_flags`.
	///
	/// Flags.
	pub(crate) ifru_flags: c_short,
	
	/// `ifr_metric`, `ifr_ifindex`, `ifr_bandwidth` or `ifr_qlen`.
	///
	/// Metric, `NetworkInterfaceIndex` (non-zero), link bandwidth or queue length.
	pub(crate) ifru_ivalue: c_int,
	
	/// `ifr_mtu`.
	///
	/// Maximum Transmission Unit (MTU).
	pub(crate) ifru_mtu: c_int,
	
	/// `ifr_map`.
	///
	/// Device Map.
	pub(crate) ifru_map: ifmap,
	
	/// `ifr_slave`.
	///
	/// Slave Device by `NetworkInterfaceName`.
	pub(crate) ifru_slave: [c_char; IFNAMSIZ],
	
	/// `ifr_newname`.
	///
	/// New `NetworkInterfaceIndex` name.
	pub(crate) ifru_newname: [c_char; IFNAMSIZ],
	
	/// `ifr_data`.
	///
	/// Data for use by network interface.
	pub(crate) ifru_data: *mut c_void,
	
	/// `ifr_settings`.
	///
	/// Device or protocol settings.
	pub(crate) ifru_settings: if_settings,
}

impl Default for ifreq_ifru
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ifreq_ifru
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "ifreq_ifru {{ union }}")
	}
}
