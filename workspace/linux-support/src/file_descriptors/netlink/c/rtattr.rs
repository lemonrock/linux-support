// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Route netlink attribute header.
///
/// Followed by attribute value.
#[repr(C)]
pub(crate) struct rtattr
{
	rta_len: u16,
	
	/// Starts `IFLA_*`.
	///
	/// See man 7 rtnetlink.
	///
	/// The following are valid for messages of type `RTM_NEWLINK`, `RTM_DELLINK` and `RTM_GETLINK`:-
	///
	/// * `IFLA_UNSPEC`      -                  unspecified.
	/// * `IFLA_ADDRESS`     hardware address   interface L2 address
	/// * `IFLA_BROADCAST`   hardware address   L2 broadcast address.
	/// * `IFLA_IFNAME`      asciiz string      Device name.	Size including `NUL` should not exceed `IF_NAMESIZE` (16).
	/// * `IFLA_MTU`         unsigned int       MTU of the device.
	/// * `IFLA_LINK`        int                Link type.
	/// * `IFLA_QDISC`       asciiz string      Queueing discipline.
	/// * `IFLA_STATS`       see below          Interface Statistics.
	/// And many others not well documented in the Linux header `if_link.h`.
	///
	/// The following are valid for messages of type `RTM_NEWADDR`, `RTM_DELADDR` and `RTM_GETADDR`:-
	///
	/// * `IFA_UNSPEC`      -                      unspecified.
	/// * `IFA_ADDRESS`     raw protocol address   interface address
	/// * `IFA_LOCAL`       raw protocol address   local address
	/// * `IFA_LABEL`       asciiz string          name of the interface	Size including `NUL` should not exceed `IF_NAMESIZE` (16).
	/// * `IFA_BROADCAST`   raw protocol address   broadcast address.
	/// * `IFA_ANYCAST`     raw protocol address   anycast address
	/// * `IFA_CACHEINFO`   struct ifa_cacheinfo   Address information.
	/// * `IFA_FLAGS`
	/// * `IFA_MULTICAST`
	/// * `IFA_RT_PRIORITY` unsigned int           Priority or metric for prefix route.
	/// * `IFA_TARGET_NETNSID`
	/// The values starting `IFLA_*` ***overlap*** with those starting `IFA_*` ie `IFLA_ADDRESS` and `IFA_ADDRESS` are the same value!
	pub(crate) rta_type: u16,
}

/// Link (`IFLA_*`).
///
/// See Linux header `if_link.h`.
impl rtattr
{
	#[inline(always)]
	pub(super) fn get_attribute_value_link_unspecified(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFLA_UNSPEC);
		
		self.get_attribute_value_unspecified()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_layer_2_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFLA_ADDRESS);
		
		self.get_attribute_value_hardware_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_layer_2_broadcast_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFLA_BROADCAST);
		
		self.get_attribute_value_hardware_address()
	}
	
	/// Maximum Transmission Unit (MTU).
	#[inline(always)]
	pub(super) fn get_attribute_value_maximum_transmission_unit(&self) -> Result<u32, TryFromSliceError>
	{
		debug_assert_eq!(self.rta_type, IFLA_MTU);
		
		self.get_attribute_value_unsigned_integer()
	}
	
	/// Device name (interface name).
	#[inline(always)]
	pub(super) fn get_attribute_value_device_name(&self) -> Result<InterfaceName, String>
	{
		debug_assert_eq!(self.rta_type, IFLA_IFNAME);
		
		self.get_attribute_value_interface_name()
	}
	
	/// Link type.
	#[inline(always)]
	pub(super) fn get_attribute_value_link_type(&self) -> Result<i32, TryFromSliceError>
	{
		debug_assert_eq!(self.rta_type, IFLA_LINK);
		
		self.get_attribute_value_signed_integer()
	}
	
	/// Queueing discipline.
	#[inline(always)]
	pub(super) fn get_attribute_value_queueing_discipline(&self) -> Result<&CStr, FromBytesWithNulError>
	{
		debug_assert_eq!(self.rta_type, IFLA_QDISC);
		
		self.get_attribute_value_asciiz_string()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_interface_statistics(&self) -> Result<&rtnl_link_stats64, &'static str>
	{
		debug_assert_eq!(self.rta_type, IFLA_STATS);
		
		let attribute_value = self.attribute_value();
		if attribute_value.len() != size_of::<rtnl_link_stats64>()
		{
			Err("Invalid length for rtnl_link_stats64")
		}
		else
		{
			Ok(unsafe { & * (attribute_value.as_ptr() as *const rtnl_link_stats64) })
		}
	}
	
	#[inline(always)]
	fn get_attribute_value_hardware_address(&self) -> &[u8]
	{
		if cfg!(debug_assertions)
		{
			match self.rta_type
			{
				IFLA_ADDRESS | IFLA_BROADCAST => (),
				
				_ => debug_assert!(false, "self.rta_type {} is not one of IFLA_ADDRESS or IFLA_BROADCAST"),
			}
		}
		self.attribute_value()
	}
}

/// Address (`IFA_*`).
///
/// See Linux header `if_addr.h`.
impl rtattr
{
	#[inline(always)]
	pub(super) fn get_attribute_value_address_unspecified(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_UNSPEC);
		
		self.get_attribute_value_unspecified()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_interface_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_ADDRESS);
		
		self.get_attribute_value_raw_protocol_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_local_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_LOCAL);
		
		self.get_attribute_value_raw_protocol_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_name_of_interface(&self) -> Result<InterfaceName, String>
	{
		debug_assert_eq!(self.rta_type, IFA_LABEL);
		
		self.get_attribute_value_interface_name()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_broadcast_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_BROADCAST);
		
		self.get_attribute_value_raw_protocol_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_anycast_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_ANYCAST);
		
		self.get_attribute_value_raw_protocol_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_cache_information(&self) -> Result<&ifa_cacheinfo, String>
	{
		debug_assert_eq!(self.rta_type, IFA_CACHEINFO);
		
		const Size: usize = size_of::<ifa_cacheinfo>();
		
		let attribute_value = self.attribute_value();
		let length = attribute_value.len();
		if length != Size
		{
			Err(format!("Invalid length {} for ifa_cacheinfo does not match size {} ", length, Size))
		}
		else
		{
			Ok(unsafe { & * (attribute_value.as_ptr() as *const ifa_cacheinfo) })
		}
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_multicast_address(&self) -> &[u8]
	{
		debug_assert_eq!(self.rta_type, IFA_MULTICAST);
		
		self.get_attribute_value_raw_protocol_address()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_extended_interface_flags(&self) -> Result<ExtendedInterfaceFlags, TryFromSliceError>
	{
		debug_assert_eq!(self.rta_type, IFA_FLAGS);
		
		Ok(ExtendedInterfaceFlags::from_bits_truncate(self.get_attribute_value_unsigned_integer()?))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_route_priority(&self) -> Result<u32, TryFromSliceError>
	{
		debug_assert_eq!(self.rta_type, IFA_RT_PRIORITY);
		
		self.get_attribute_value_unsigned_integer()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_target_net_namespace_identifier(&self) -> Result<i32, TryFromSliceError>
	{
		debug_assert_eq!(self.rta_type, IFA_TARGET_NETNSID);
		
		self.get_attribute_value_signed_integer()
	}
	
	#[inline(always)]
	fn get_attribute_value_raw_protocol_address(&self) -> &[u8]
	{
		if cfg!(debug_assertions)
		{
			match self.rta_type
			{
				IFA_ADDRESS | IFA_LOCAL | IFA_BROADCAST | IFA_ANYCAST | IFA_MULTICAST => (),
				
				_ => debug_assert!(false, "self.rta_type {} is not one of IFA_ADDRESS, IFA_LOCAL, IFA_BROADCAST, IFA_ANYCAST or IFA_MULTICAST"),
			}
		}
		self.attribute_value()
	}
}

impl rtattr
{
	#[inline(always)]
	fn get_attribute_value_interface_name(&self) -> Result<InterfaceName, String>
	{
		let asciiz_string = self.get_attribute_value_asciiz_string().map_err(|error| error.to_string())?;
		let bytes = asciiz_string.to_bytes();
		let mut array_vec = ArrayVec::new();
		array_vec.try_extend_from_slice(bytes).map_err(|_| format!("Interface name length must be equal to or less than {}, not {}", InterfaceName::InterfaceNameSize, bytes.len()))?;
		Ok(InterfaceName(array_vec))
	}
	
	#[inline(always)]
	fn get_attribute_value_asciiz_string(&self) -> Result<&CStr, FromBytesWithNulError>
	{
		let value = self.attribute_value();
		CStr::from_bytes_with_nul(value)
	}
	
	#[inline(always)]
	fn get_attribute_value_unsigned_integer(&self) -> Result<u32, TryFromSliceError>
	{
		let value: [u8; 4] = self.attribute_value().try_into()?;
		Ok(u32::from_ne_bytes(value))
	}
	
	#[inline(always)]
	fn get_attribute_value_signed_integer(&self) -> Result<i32, TryFromSliceError>
	{
		let value: [u8; 4] = self.attribute_value().try_into()?;
		Ok(i32::from_ne_bytes(value))
	}
	
	#[inline(always)]
	fn get_attribute_value_unspecified(&self) -> &[u8]
	{
		self.attribute_value()
	}
	
	#[inline(always)]
	fn attribute_value(&self) -> &[u8]
	{
		unsafe { from_raw_parts(self.RTA_DATA(), self.RTA_PAYLOAD()) }
	}
	
	#[inline(always)]
	pub(super) fn attribute_value_mut(&mut self) -> &[u8]
	{
		unsafe { from_raw_parts_mut(self.RTA_DATA() as *mut u8, self.RTA_PAYLOAD()) }
	}
	
	const RTA_ALIGNTO: usize = 4;
	
	const RTA_HDRLEN: usize = size_of::<Self>();
	
	#[inline(always)]
	const fn RTA_ALIGN(length: usize) -> usize
	{
		(length + Self::RTA_ALIGNTO - 1) & !(Self::RTA_ALIGNTO - 1)
	}
	
	#[inline(always)]
	const fn RTA_LENGTH(length: usize) -> usize
	{
		Self::RTA_ALIGN(Self::RTA_HDRLEN) + length
	}
	
	#[inline(always)]
	const fn RTA_SPACE(length: usize) -> usize
	{
		Self::RTA_ALIGN(Self::RTA_LENGTH(length))
	}
	
	/// Pointer to start of payload.
	#[inline(always)]
	const fn RTA_DATA(&self) -> *const u8
	{
		unsafe { (self as *const Self as *const u8).add(Self::RTA_LENGTH(0)) }
	}
	
	#[inline(always)]
	fn RTA_DATALEN(&self) -> usize
	{
		self.length() - Self::RTA_HDRLEN
	}
	
	#[inline(always)]
	fn RTA_DATAEND(&self) -> *const u8
	{
		unsafe { (self as *const Self as *const u8).add(self.length()) }
	}
	
	/// Size of payload.
	#[inline(always)]
	fn RTA_PAYLOAD(&self) -> usize
	{
		self.length() - Self::RTA_LENGTH(0)
	}
	
	#[inline(always)]
	fn RTA_OK(remaining_length: usize, might_be_invalid_pointer: *const Self) -> bool
	{
		if remaining_length < Self::RTA_HDRLEN
		{
			return false
		}
		
		let our_length = unsafe { & * might_be_invalid_pointer }.length();
		
		our_length >= Self::RTA_HDRLEN && our_length <= remaining_length
	}
	
	#[inline(always)]
	fn RTA_NEXT(&self, remaining_length: &mut usize) -> *const Self
	{
		let length = Self::RTA_ALIGN(self.length());
		
		*remaining_length = (*remaining_length) - length;
		
		unsafe { (self as *const Self as *const u8).add(length) as *const Self }
	}
	
	/// `RTA_NEXT` in `musl`.
	#[inline(always)]
	pub(crate) fn next(&self) -> *const Self
	{
		let length = Self::RTA_ALIGN(self.length());
		unsafe { (self as *const Self as *const u8).add(length) as *const Self }
	}
	
	/// `RTA_OK` in `musl`.
	#[inline(always)]
	pub(crate) const fn ok(this: *const Self, end: usize) -> bool
	{
		(end - (this as usize)) >= Self::RTA_HDRLEN
	}
	
	/// `NLMSG_RTAOK` in `musl`.
	#[inline(always)]
	pub(crate) fn NLMSG_RTAOK(this: *const Self, header: &nlmsghdr) -> bool
	{
		Self::ok(this, header.NLMSG_DATALEN())
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.rta_len as usize
	}
}
