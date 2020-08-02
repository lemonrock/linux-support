// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Route netlink attribute header.
///
/// Followed by attribute value.
#[repr(C)]
pub struct rtattr<NAT: NetlinkAttributeType>
{
	rta_len: u16,
	
	rta_type: u16,

	marker: PhantomData<NAT>,
}

impl<NAT: NetlinkAttributeType> rtattr<NAT>
{
	#[inline(always)]
	pub(crate) fn type_(&self) -> (bool, bool, NAT)
	{
		(
			self.rta_type & (nlattr::NLA_F_NESTED) != 0,
			self.rta_type & (nlattr::NLA_F_NET_BYTEORDER) != 0,
			unsafe { transmute(self.rta_type & nlattr::NLA_TYPE_MASK) }
		)
	}
	
	#[inline(always)]
	pub(crate) fn get_attribute_value_nested(&self) -> &[u8]
	{
		let (is_nested, is_in_network_byte_order, _is) = self.type_();
		debug_assert!(is_nested, "Is not nested");
		debug_assert!(!is_in_network_byte_order, "Is in network byte order");
		
		self.attribute_value()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_network_interface_name(&self) -> Result<NetworkInterfaceName, String>
	{
		let asciiz_string = self.get_attribute_value_asciiz_string().map_err(|error| error.to_string())?;
		let bytes = asciiz_string.to_bytes();
		NetworkInterfaceName::from_bytes(bytes).map_err(|error| format!("{}", error))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_network_interface_alias(&self) -> Result<NetworkInterfaceAlias, String>
	{
		let asciiz_string = self.get_attribute_value_asciiz_string().map_err(|error| error.to_string())?;
		let bytes = asciiz_string.to_bytes();
		NetworkInterfaceAlias::from_bytes(bytes).map_err(|error| format!("{}", error))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_queue_count(&self) -> Result<QueueCount, String>
	{
		let value = self.get_attribute_value_non_zero_u32()?;
		QueueCount::try_from(value).map_err(|error| error.to_string())
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_maximum_transmission_unit(&self) -> Result<MaximumTransmissionUnit, TryFromSliceError>
	{
		self.get_attribute_value_u32().map(|value| MaximumTransmissionUnit(value))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_net_dev_group(&self) -> Result<NetworkDeviceGroup, TryFromSliceError>
	{
		self.get_attribute_value_u32().map(|value| NetworkDeviceGroup(value))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_net_namespace_identifier(&self) -> Result<i32, TryFromSliceError>
	{
		self.get_attribute_value_i32()
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_asciiz_string(&self) -> Result<&CStr, FromBytesWithNulError>
	{
		let value = self.attribute_value();
		CStr::from_bytes_with_nul(value)
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_network_interface_index(&self) -> Result<NetworkInterfaceIndex, String>
	{
		NetworkInterfaceIndex::try_from(self.get_attribute_value_non_zero_u32()).map_err(|error| error.to_string())
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_non_zero_u32(&self) -> Result<NonZeroU32, String>
	{
		let value = self.get_attribute_value_u32().map_err(|error| error.to_string())?;
		NonZeroU32::new(value).ok_or(format!("Was zero"))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_bool(&self) -> Result<bool, TryFromSliceError>
	{
		self.get_attribute_value_u8().map(|value| value != 0)
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_u8(&self) -> Result<u8, TryFromSliceError>
	{
		let value: [u8; 1] = self.attribute_value().try_into()?;
		Ok(u8::from_ne_bytes(value))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_u32(&self) -> Result<u32, TryFromSliceError>
	{
		let value: [u8; 4] = self.attribute_value().try_into()?;
		Ok(u32::from_ne_bytes(value))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_i32(&self) -> Result<i32, TryFromSliceError>
	{
		let value: [u8; 4] = self.attribute_value().try_into()?;
		Ok(i32::from_ne_bytes(value))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_struct_cloned<T: Sized + Clone>(&self) -> Result<T, String>
	{
		self.get_attribute_value_struct::<T>().map(|reference| reference.clone())
	}
	
	#[inline(always)]
	fn get_attribute_value_struct<T: Sized>(&self) -> Result<&T, String>
	{
		let attribute_value = self.attribute_value();
		let length = attribute_value.len();
		let known_size = size_of::<T>();
		// Can be greater than if Linux has subsequently added more fields.
		if length >= known_size
		{
			Ok(unsafe { & * (attribute_value.as_ptr() as *const T) })
		}
		else
		{
			Err(format!("Invalid length {} for T does not match size {} ", length, known_size))
		}
	}
	
	#[inline(always)]
	fn attribute_value(&self) -> &[u8]
	{
		unsafe { from_raw_parts(self.RTA_DATA(), self.RTA_PAYLOAD()) }
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
	
	/// Pointer to start of payload.
	#[inline(always)]
	fn RTA_DATA(&self) -> *const u8
	{
		unsafe { (self as *const Self as *const u8).add(Self::RTA_LENGTH(0)) }
	}
	
	/// Size of payload.
	#[inline(always)]
	fn RTA_PAYLOAD(&self) -> usize
	{
		self.length() - Self::RTA_LENGTH(0)
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
	pub(crate) const fn ok(this: *const Self, end: *const Self) -> bool
	{
		(end as usize) - (this as usize) >= Self::RTA_HDRLEN
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.rta_len as usize
	}
	
	#[inline(always)]
	fn debug_assert_is(&self, is: NTA)
	{
		debug_assert_eq!(self.type_(), (false, false, is))
	}
	
	#[inline(always)]
	fn debug_assert_is_not_nested_and_is_in_native_byte_order(&self)
	{
		let (is_nested, is_in_network_byte_order, _is) = self.type_();
		debug_assert!(!is_nested, "Is nested");
		debug_assert!(!is_in_network_byte_order, "Is network byte order")
	}
}

/// Link (`IFLA_*`).
///
/// See Linux header `if_link.h`.
///
/// See man 7 rtnetlink.
impl rtattr<IFLA>
{
	#[inline(always)]
	pub(super) fn get_attribute_value_operational_status(&self) -> Result<IF_OPER, TryFromSliceError>
	{
		self.debug_assert_is(IFLA::IFLA_OPERSTATE);
		
		Ok(unsafe { transmute(self.get_attribute_value_u8()?) })
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_link_mode(&self) -> Result<IF_LINK_MODE, TryFromSliceError>
	{
		self.debug_assert_is(IFLA::IFLA_LINKMODE);
		
		Ok(unsafe { transmute(self.get_attribute_value_u8()?) })
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_physical_identifier(&self) -> Result<PhysicalIdentifier, PhysicalIdentifierFromBytesError>
	{
		self.debug_assert_is_not_nested_and_is_in_native_byte_order();
		
		PhysicalIdentifier::try_from(self.attribute_value())
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_hardware_address(&self) -> &[u8]
	{
		self.debug_assert_is_not_nested_and_is_in_native_byte_order();
		
		use self::IFLA::*;
		debug_assert!(matches!(self.type_().2, IFLA_ADDRESS | IFLA_BROADCAST | IFLA_PERM_ADDRESS), "self.type_().2 {} is not one of IFLA_ADDRESS, IFLA_BROADCAST or IFLA_PERM_ADDRESS", self.type_().2);
		
		self.attribute_value()
	}
}

/// Address (`IFA_*`).
///
/// See Linux header `if_addr.h`.
///
/// See man 7 rtnetlink.
impl rtattr<IFA>
{
	#[inline(always)]
	pub(super) fn get_attribute_value_extended_interface_flags(&self) -> Result<ExtendedInterfaceFlags, TryFromSliceError>
	{
		self.debug_assert_is(IFA::IFA_FLAGS);
		
		Ok(ExtendedInterfaceFlags::from_bits_truncate(self.get_attribute_value_u32()?))
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_raw_protocol_address(&self) -> &[u8]
	{
		self.debug_assert_is_not_nested_and_is_in_native_byte_order();
		
		use self::IFA::*;
		debug_assert!(matches!(self.type_().2, IFA_ADDRESS | IFA_LOCAL | IFA_BROADCAST | IFA_ANYCAST | IFA_MULTICAST), "self.type_().2 {} is not one of IFA_ADDRESS, IFA_LOCAL, IFA_BROADCAST, IFA_ANYCAST or IFA_MULTICAST", self.type_().2);
		
		self.attribute_value()
	}
}

impl rtattr<IFLA_XDP>
{
	#[inline(always)]
	pub(super) fn get_attribute_value_attached(&self) -> Result<XDP_ATTACHED, TryFromSliceError>
	{
		self.debug_assert_is(IFLA_XDP::IFLA_XDP_ATTACHED);
		
		Ok(unsafe { transmute(self.get_attribute_value_u8()?) })
	}
	
	#[inline(always)]
	pub(super) fn get_attribute_value_program_identifier(&self) -> Result<ExtendedBpfProgramIdentifier, TryFromSliceError>
	{
		self.debug_assert_is_not_nested_and_is_in_native_byte_order();
		
		use self::IFLA_XDP::*;
		debug_assert!(matches!(self.type_().2, IFLA_XDP_PROG_ID | IFLA_XDP_SKB_PROG_ID | IFLA_XDP_DRV_PROG_ID | IFLA_XDP_HW_PROG_ID), "self.type_().2 {} is not one of IFLA_XDP_PROG_ID, IFLA_XDP_SKB_PROG_ID, IFLA_XDP_DRV_PROG_ID or IFLA_XDP_HW_PROG_ID", self.type_().2);
		
		self.get_attribute_value_u32().map(|value| ExtendedBpfProgramIdentifier::from(value))
	}
}
