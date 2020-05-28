// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetAddressMessageData<IPA: InternetProtocolAddress>
{
	interface_flags: InterfaceFlags,
	
	/// `0..=32` for IPv4.
	/// `0..=128` for IPv6.
	mask_length_in_bits: u8,
	
	/// Zone identifier (interface id) used for link-local addressing.
	/// Known as `sin6_scope_id` in the libc struct `sockaddr_in6`.
	/// See <https://tools.ietf.org/html/rfc6874>.
	/// Examples of the value (after the `%`) are:-
	///
	/// * `fe80::1:2:3:4%9`
	/// * `fe80::1:2:3:4%eth0`
	///
	/// This is the index of the interface with the associated address(es).
	///
	/// Only relevant to IPv6 if the `IN6_IS_ADDR_LINKLOCAL()` or `IN6_IS_ADDR_MC_LINKLOCAL()` macros are true.
	interface_index: Option<NetworkInterfaceIndex>,
	
	scope: rt_scope,
	
	/// destination address for a point-to-point interface.
	/// local address for a regular interface.
	address: Option<IPA>,

	local_address: Option<IPA>,

	label: Option<InterfaceName>,
	
	broadcast_address: Option<IPA>,
	
	anycast_address: Option<IPA>,
	
	cache_information: Option<ifa_cacheinfo>,
	
	/// If present, overrides `interface_flags`.
	extended_interface_flags: Option<ExtendedInterfaceFlags>,
	
	multicast_address: Option<IPA>,
	
	route_priority: Option<u32>,
	
	target_net_namespace_identifier: Option<i32>,
}

#[allow(missing_docs)]
impl<IPA: InternetProtocolAddress> GetAddressMessageData<IPA>
{
	/// ?Such as `eth0`?
	#[inline(always)]
	pub fn interface_name(&self) -> Option<&InterfaceName>
	{
		self.label.as_ref()
	}
	
	/// ?Use this for an IPv6 link-local address zone identifier?
	#[inline(always)]
	pub fn interface_index(&self) -> NetworkInterfaceIndex
	{
		self.interface_index.unwrap()
	}
	
	/// Scope.
	#[inline(always)]
	pub fn address_scope(&self) -> rt_scope
	{
		self.scope
	}
	
	#[inline(always)]
	pub fn interface_flags(&self) -> ExtendedInterfaceFlags
	{
		if let Some(extended_interface_flags) = self.extended_interface_flags
		{
			extended_interface_flags
		}
		else
		{
			self.interface_flags.into()
		}
	}
	
	#[inline(always)]
	pub fn local_address_and_destination_address_for_point_to_point(&self) -> Option<(Either<&IPA, InternetProtocolAddressWithMask<IPA>>, Option<&IPA>)>
	{
		#[inline(always)]
		fn to_address_or_masked_address<IPA: InternetProtocolAddress>(address: &IPA, mask_length_in_bits: u8) -> Either<&IPA, InternetProtocolAddressWithMask<IPA>>
		{
			if mask_length_in_bits == 0
			{
				Left(address)
			}
			else if mask_length_in_bits <= IPA::InclusiveMaximumPrefixLength
			{
				Right(InternetProtocolAddressWithMask
				{
					internet_protocol_address: address.clone(),
					mask_length_in_bits: unsafe { NonZeroU8::new_unchecked(mask_length_in_bits) },
				})
			}
			else
			{
				panic!("Invalid netmask")
			}
		}
		
		match (self.local_address.as_ref(), self.address.as_ref())
		{
			(Some(local_address), Some(destination_address)) => if local_address == destination_address
			{
				Some((to_address_or_masked_address(local_address, self.mask_length_in_bits), None))
			}
			else
			{
				Some((to_address_or_masked_address(local_address, self.mask_length_in_bits), Some(destination_address)))
			}
			
			(None, Some(local_address)) => Some((to_address_or_masked_address(local_address, self.mask_length_in_bits), None)),
			
			(Some(local_address), None) => Some((to_address_or_masked_address(local_address, self.mask_length_in_bits), None)),
			
			(None, None) => None,
		}
	}
	
	#[inline(always)]
	pub fn broadcast_address(&self) -> Option<&IPA>
	{
		self.broadcast_address.as_ref()
	}
	
	#[inline(always)]
	pub fn multicast_address(&self) -> Option<&IPA>
	{
		self.multicast_address.as_ref()
	}
	
	#[inline(always)]
	pub fn anycast_address(&self) -> Option<&IPA>
	{
		self.anycast_address.as_ref()
	}
	
	#[inline(always)]
	pub fn cache_information(&self) -> Option<&ifa_cacheinfo>
	{
		self.cache_information.as_ref()
	}
	
	#[inline(always)]
	pub fn route_priority(&self) -> Option<u32>
	{
		self.route_priority
	}
	
	#[inline(always)]
	pub fn target_net_namespace_identifier(&self) -> Option<i32>
	{
		self.target_net_namespace_identifier
	}
	
	#[inline(always)]
	fn new(message_header: &ifaddrmsg) -> Self
	{
		Self
		{
			interface_flags: message_header.ifa_flags,
			mask_length_in_bits: message_header.ifa_prefixlen,
			interface_index: message_header.ifa_index,
			scope: message_header.ifa_scope,
			address: None,
			local_address: None,
			label: None,
			broadcast_address: None,
			anycast_address: None,
			cache_information: None,
			extended_interface_flags: None,
			multicast_address: None,
			route_priority: None,
			target_net_namespace_identifier: None
		}
	}
}
