// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Field ordering based on order as generated by `inet6_fill_ifaddr()` in Linux source `addrconf.c` (for Internet Protocol version 6).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GetAddressProcessingMessageStateUnicastCommon<IPA: InternetProtocolAddress>
{
	pub(crate) common: GetAddressProcessingMessageStateCommon,
	
	/// If this is a normal link:-
	///
	/// * This should be ignored, but
	/// * `address` should have the same value.
	///
	/// Only valid if there is a point-to-point peer (`IFF_POINTTOPOINT` is set in `net_device_flags`), in which case:-
	///
	/// * This (`local_address`) is the source address;
	/// * This (`address`) is the peer (destination) address;
	/// * `common.mask_length_in_bits` should be `Some(IPA::InclusiveMaximumPrefixLength)`.
	pub(crate) local_address: Option<IPA>,
	
	/// If this is a normal link:-
	///
	/// * This is the source address;
	/// * `local_address` should have the same value.
	///
	/// If there is a a point-to-point peer (`IFF_POINTTOPOINT` is set in `net_device_flags`):-
	///
	/// * This is the peer (destination) address;
	/// * `local_address` is the source address;
	/// * `common.mask_length_in_bits` should be `Some(IPA::InclusiveMaximumPrefixLength)`.
	pub(crate) address: Option<IPA>,
	
	pub(crate) route_priority: Option<NonZeroU32>,
	
	/// Mandatory.
	pub(crate) extended_interface_flags: Option<ExtendedInterfaceFlags>,
}

#[allow(missing_docs)]
impl<IPA: InternetProtocolAddress> GetAddressProcessingMessageStateUnicastCommon<IPA>
{
	#[inline(always)]
	pub(crate) fn new(message_header: &ifaddrmsg) -> Result<Self, String>
	{
		Ok
		(
			Self
			{
				common: GetAddressProcessingMessageStateCommon::new::<IPA>(message_header)?,
				local_address: None,
				address: None,
				route_priority: None,
				extended_interface_flags: None,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<GetAddressMessageDataUnicastCommon<IPA>, String>
	{
		let common = self.common.to_processed_message::<IPA>()?;
		
		let mask_length_in_bits = common.mask_length_in_bits;
		
		// Based on convuluted logic in musl libc `getifaddrs.c`.
		let source_address_and_point_to_point_peer_destination_address = match (self.address, self.local_address)
		{
			(None, None) => None,
			
			(Some(address), None) => Some((address, None)),
			
			(None, Some(local_address)) => Some((local_address, None)),
			
			(Some(address), Some(local_address)) => Some((local_address, Some(address))),
		};
		
		Ok
		(
			GetAddressMessageDataUnicastCommon
			{
				common,
				
				source_address_and_point_to_point_peer_destination_address,
				
				route_priority: self.route_priority,
				
				extended_interface_flags:
				{
					let extended_interface_flags = self.extended_interface_flags.ok_or(format!("Linux kernel bug - missing extended_interface_flags"))?;
					
					if likely!(extended_interface_flags.contains(ExtendedInterfaceFlags::from(self.common.interface_flags)))
					{
						extended_interface_flags
					}
					else
					{
						return Err(format!("Linux kernel bug - missing interface_flags is not a subset of extended_interface_flags"))
					}
				},
			}
		)
	}
}
