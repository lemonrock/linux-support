// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct GetAddressRouteMessageProcessor<IPA: InternetProtocolAddress>(PhantomData<IPA>);

impl<IPA: InternetProtocolAddress> RouteMessageProcessor for GetAddressRouteMessageProcessor<IPA>
{
	type Header = ifaddrmsg;
	
	const ExpectedMessageType: RouteNetlinkMessageType = RouteNetlinkMessageType::NEWLINK;
	
	type ProcessingMessageState = GetAddressMessageData<IPA>;
	
	type ProcessedMessage = GetAddressMessageData<IPA>;
	
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
	fn process_message_attribute(&self, message_attribute: &rtattr, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>
	{
		#[inline(always)]
		fn set_address_field<IPA: InternetProtocolAddress, F: FnOnce(&rtattr) -> &[u8]>(field: &mut Option<IPA>, message_attribute: &rtattr, attribute: F) -> Result<(), String>
		{
			set_field_error(field, message_attribute, |message_attribute| IPA::from_bytes(attribute(message_attribute)))
		}
		
		#[inline(always)]
		fn set_field<Field, F: FnOnce(&rtattr) -> Field>(field: &mut Option<Field>, message_attribute: &rtattr, attribute: F) -> Result<(), String>
		{
			set_field_error::<Field, Infallible, _>(field, message_attribute, |message_attribute| Ok(attribute(message_attribute)))
		}
		
		#[inline(always)]
		fn set_field_error<Field, Error: ToString, F: FnOnce(&rtattr) -> Result<Field, Error>>(field: &mut Option<Field>, message_attribute: &rtattr, attribute: F) -> Result<(), String>
		{
			if field.is_some()
			{
				return Err(format!("field already populated; duplicate rtattr"))
			}
			
			*field.as_mut().unwrap() = attribute(message_attribute).map_err(|error| error.to_string())?;
			Ok(())
		}
		
		match message_attribute.rta_type
		{
			IFA_UNSPEC => (),
			
			IFA_ADDRESS=> set_address_field(&mut processing_message_state.address, message_attribute, rtattr::get_attribute_value_interface_address)?,
			
			IFA_LOCAL => set_address_field(&mut processing_message_state.local_address, message_attribute, rtattr::get_attribute_value_local_address)?,
			
			IFA_LABEL=> set_field_error(&mut processing_message_state.label, message_attribute, |message_attribute| message_attribute.get_attribute_value_name_of_interface())?,
			
			IFA_BROADCAST => set_address_field(&mut processing_message_state.broadcast_address, message_attribute, rtattr::get_attribute_value_broadcast_address)?,
			
			IFA_ANYCAST=> set_address_field(&mut processing_message_state.anycast_address, message_attribute, rtattr::get_attribute_value_anycast_address)?,
			
			IFA_CACHEINFO=> set_field_error::<_, String, _>(&mut processing_message_state.cache_information, message_attribute, |message_attribute| Ok(message_attribute.get_attribute_value_cache_information()?.clone()))?,
			
			IFA_FLAGS => set_field_error(&mut processing_message_state.extended_interface_flags, message_attribute, rtattr::get_attribute_value_extended_interface_flags)?,
			
			IFA_MULTICAST=> set_address_field(&mut processing_message_state.multicast_address, message_attribute, rtattr::get_attribute_value_multicast_address)?,
			
			IFA_RT_PRIORITY => set_field_error(&mut processing_message_state.route_priority, message_attribute, rtattr::get_attribute_value_route_priority)?,
			
			IFA_TARGET_NETNSID => set_field_error(&mut processing_message_state.target_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_target_net_namespace_identifier)?,
			
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
