// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GetInternetProtocolVersion6MulticastAddressProcessingMessageState
{
	common: GetAddressProcessingMessageStateCommon,
	
	pub(crate) multicast_address: Option<IPA>,
}

#[allow(missing_docs)]
impl<IPA: InternetProtocolAddress> GetAddressProcessingMessageState<IPA>
{
	#[inline(always)]
	pub(crate) fn new(message_header: &ifaddrmsg) -> Result<Self, String>
	{
		if unlikely!(message_header.ifi_family != IPA::AddressFamily)
		{
			return Err(format!("Linux kernel bug - ifi_family is not AF_INET or AF_INET6"))
		}
		Ok
		(
			Self
			{
				interface_flags: message_header.ifa_flags,
				mask_length_in_bits: message_header.ifa_prefixlen,
				network_interface_index: message_header.ifa_index.ok_or(format!("Linux kernel bug - missing network interface index"))?,
				address_scope: message_header.ifa_scope,
				target_net_namespace_identifier: None,
				local_address: None,
				address: None,
				route_priority: None,
				cache_information: None,
				extended_interface_flags: None,
				interface_name: None,
				broadcast_address: None,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<GetAddressMessageData<IPA>, String>
	{
		Ok
		(
			GetAddressMessageData
			{
				mask_length_in_bits: self.mask_length_in_bits,
				
				network_interface_index: self.network_interface_index,
				
				address_scope: self.address_scope,
				
				target_net_namespace_identifier: self.target_net_namespace_identifier,
				
				local_address: self.local_address,
				
				address: self.address,
				
				route_priority: self.route_priority,
				
				cache_information: self.cache_information.ok_or(format!("Linux kernel bug - missing cache_information"))?,
				
				extended_interface_flags:
				{
					let extended_interface_flags = self.extended_interface_flags.ok_or(format!("Linux kernel bug - missing extended_interface_flags"))?;
					
					if likely!(extended_interface_flags.contains(ExtendedInterfaceFlags::from(self.interface_flags)))
					{
						extended_interface_flags
					}
					else
					{
						return Err(format!("Linux kernel bug - missing interface_flags is not a subset of extended_interface_flags"))
					}
				},
				
				interface_name: self.interface_name,
				
				broadcast_address: self.broadcast_address,
			}
		)
	}
}
