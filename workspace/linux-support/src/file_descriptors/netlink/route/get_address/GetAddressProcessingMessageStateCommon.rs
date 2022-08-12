// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GetAddressProcessingMessageStateCommon
{
	interface_flags: InterfaceFlags,
	
	mask_length_in_bits: u8,
	
	network_interface_index: NetworkInterfaceIndex,
	
	address_scope: rt_scope,
	
	pub(crate) target_net_namespace_identifier: Option<NetNamespaceIdentifier>,
	
	/// Mandatory.
	pub(crate) cache_information: Option<ifa_cacheinfo>,
}

#[allow(missing_docs)]
impl GetAddressProcessingMessageStateCommon
{
	#[inline(always)]
	pub(crate) fn new<IPA: InternetProtocolAddress>(message_header: &ifaddrmsg) -> Result<Self, String>
	{
		if unlikely!(message_header.ifa_family != IPA::AddressFamily)
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
				cache_information: None,
			}
		)
	}
	
	
	#[inline(always)]
	pub(crate) fn to_processed_message<IPA: InternetProtocolAddress>(self) -> Result<GetAddressMessageDataCommon, String>
	{
		let (temporary_address_valid_lifetime, temporary_address_preferred_lifetime, temporary_address_created_timestamp, temporary_address_updated_timestamp) = self.cache_information()?;
		
		Ok
		(
			GetAddressMessageDataCommon
			{
				mask_length_in_bits: self.mask_length_in_bits::<IPA>()?,
				
				network_interface_index: self.network_interface_index,
				
				address_scope: self.address_scope,
				
				target_net_namespace_identifier: self.target_net_namespace_identifier,
				
				temporary_address_valid_lifetime,
				
				temporary_address_preferred_lifetime,
				
				temporary_address_created_timestamp,
				
				temporary_address_updated_timestamp,
			}
		)
	}
	
	#[inline(always)]
	fn mask_length_in_bits<IPA: InternetProtocolAddress>(&self) -> Result<Option<NonZeroU8>, String>
	{
		let mask_length_in_bits = self.mask_length_in_bits;
		if mask_length_in_bits == 0
		{
			Ok(None)
		}
		else if mask_length_in_bits <= IPA::InclusiveMaximumPrefixLength
		{
			Ok(Some(new_non_zero_u8(mask_length_in_bits)))
		}
		else
		{
			Err(format!("Invalid netmask {}", mask_length_in_bits))
		}
	}
	
	#[inline(always)]
	fn cache_information(&self) -> Result<(InternetProtocolAddressLifetime, InternetProtocolAddressLifetime, CacheTimestampInHundrethsOfSeconds, CacheTimestampInHundrethsOfSeconds), String>
	{
		let ifa_cacheinfo { ifa_preferred: ref ifa_preferred, ref ifa_valid, cstamp: ref c_stamp, tstamp: ref t_stamp } = self.cache_information.as_ref().ok_or(format!("Linux kernel bug - missing cache_information"))?;
		Ok((*ifa_preferred, *ifa_valid, *c_stamp, *t_stamp))
	}
}
