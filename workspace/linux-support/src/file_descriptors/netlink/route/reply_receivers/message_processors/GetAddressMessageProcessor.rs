// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct GetAddressMessageProcessor<IPA: InternetProtocolAddress>(PhantomData<IPA>);

impl<IPA: InternetProtocolAddress> MessageProcessor for GetAddressMessageProcessor<IPA>
{
	type Header = ifaddrmsg;
	
	type ProcessingMessageState = GetAddressMessageData<IPA>;
	
	type ProcessedMessage = GetAddressMessageData<IPA>;
	
	type NAT = IFA;
	
	#[inline(always)]
	fn process_message_header(&self, message_header: &Self::Header) -> Result<Option<Self::ProcessingMessageState>, String>
	{
		if message_header.ifa_family != IPA::AddressFamily
		{
			return Ok(None)
		}
		
		Ok(Some(GetAddressMessageData::new(message_header)))
	}
	
	#[inline(always)]
	fn process_message_attribute(&self, message_attribute: &rtattr<Self::NAT>, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>
	{
		#[inline(always)]
		fn set_address_field<IPA: InternetProtocolAddress, F: FnOnce(&rtattr<IFA>) -> &[u8]>(field: &mut Option<IPA>, message_attribute: &rtattr<IFA>, attribute: F) -> Result<(), String>
		{
			set_field_error(field, message_attribute, |message_attribute| IPA::from_bytes(attribute(message_attribute)))
		}
		
		#[inline(always)]
		fn set_field_error<Field, Error: ToString, F: FnOnce(&rtattr<IFA>) -> Result<Field, Error>>(field: &mut Option<Field>, message_attribute: &rtattr<IFA>, attribute: F) -> Result<(), String>
		{
			*field.as_mut().ok_or(format!("field already populated; duplicate rtattr"))? = attribute(message_attribute).map_err(|error| error.to_string())?;
			Ok(())
		}
		
		use self::IFA::*;
		
		match message_attribute.type_()
		{
			(false, false, IFA_ADDRESS) => set_address_field(&mut processing_message_state.address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
		
			(false, false, IFA_LOCAL) => set_address_field(&mut processing_message_state.local_address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
			
			(false, false, IFA_LABEL) => Self::set_field_error(&mut processing_message_state.label, message_attribute, rtattr::get_attribute_value_network_interface_name)?,
			
			(false, false, IFA_BROADCAST) => set_address_field(&mut processing_message_state.broadcast_address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
			
			(false, false, IFA_ANYCAST) => set_address_field(&mut processing_message_state.anycast_address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
			
			(false, false, IFA_CACHEINFO) => Self::set_field_error(&mut processing_message_state.cache_information, message_attribute, |message_attribute| message_attribute.get_attribute_value_struct_cloned::<ifa_cacheinfo>())?,
			
			(false, false, IFA_FLAGS) => Self::set_field_error(&mut processing_message_state.extended_interface_flags, message_attribute, rtattr::get_attribute_value_extended_interface_flags)?,
			
			(false, false, IFA_MULTICAST) => set_address_field(&mut processing_message_state.multicast_address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
			
			(false, false, IFA_RT_PRIORITY) => Self::set_field_error(&mut processing_message_state.route_priority, message_attribute, rtattr::get_attribute_value_u32)?,
			
			(false, false, IFA_TARGET_NETNSID) => Self::set_field_error(&mut processing_message_state.target_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			
			(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
			
			(_, _, IFA_UNSPEC) => (),
			
			_ => (),
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn finalize(&self, processing_message_state: Self::ProcessingMessageState) -> Result<Self::ProcessedMessage, String>
	{
		Ok(processing_message_state)
	}
}

impl<IPA: InternetProtocolAddress> GetAddressMessageProcessor<IPA>
{
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self(PhantomData)
	}
	
	#[inline(always)]
	pub(crate) fn get_internet_protocol_addresses(&self, netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>) -> Result<Vec<GetAddressMessageData<IPA>>, String>
	{
		RouteNetlinkProtocol::make_request_and_get_reply_messages
		(
			netlink_socket_file_descriptor,
			self,
			Self::new_route_get_internet_protocol_addresses_message()).map_err(|error| match error
			{
				Left(messaging_parsing_errors) => format!("Message parsing errors {:?}", messaging_parsing_errors),
				
				Right(end_of_set_of_messages_error) => format!("End of set of messages errors {}", end_of_set_of_messages_error),
			}
		)
	}
	
	#[inline(always)]
	fn new_route_get_internet_protocol_addresses_message() -> NetlinkRequestMessage<ifaddrmsg>
	{
		Self::new_route_get_addresses_message(IPA::AddressFamily)
	}
	
	#[inline(always)]
	#[allow(dead_code)]
	fn new_route_get_all_addresses_message() -> NetlinkRequestMessage<ifaddrmsg>
	{
		Self::new_route_get_addresses_message(AF_UNSPEC as u8)
	}
	
	#[inline(always)]
	fn new_route_get_addresses_message(address_family: u8) -> NetlinkRequestMessage<ifaddrmsg>
	{
		let body = ifaddrmsg
		{
			// Selector field is non-zero family.
			ifa_family: address_family,
			
			// Dump Filter field is non-zero index.
			ifa_index: None,
			
			// Must all be zero (seems mad that the same struct is used for both requests and replies).
			ifa_prefixlen: unsafe { zeroed() },
			ifa_flags: unsafe { zeroed() },
			ifa_scope: unsafe { zeroed() },
		};
		RouteNetlinkProtocol::new_get_request_message(RouteNetlinkMessageType::GETADDR, NetlinkGetRequestMessageFlags::Dump, body)
	}
}
