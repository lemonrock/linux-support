// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct GetInternetProtocolVersion6AnycastAddressMessageProcessor;

impl MessageProcessor for GetInternetProtocolVersion6AnycastAddressMessageProcessor
{
	type Header = ifaddrmsg;
	
	type ProcessingMessageState = GetAddressProcessingMessageStateOtherCastInternetProtocolVersion6;
	
	type ProcessedMessage = GetInternetProtocolVersion6OtherCastAddressMessageData;
	
	type NAT = IFA;
	
	#[inline(always)]
	fn process_message_header(&self, message_header: &Self::Header) -> Result<Option<Self::ProcessingMessageState>, String>
	{
		if message_header.ifa_family != in6_addr::AddressFamily
		{
			return Ok(None)
		}
		
		Ok(Some(GetAddressProcessingMessageStateOtherCastInternetProtocolVersion6::new(message_header)?))
	}
	
	#[inline(always)]
	fn process_message_attribute(&self, message_attribute: &rtattr<Self::NAT>, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>
	{
		use self::IFA::*;
		
		match message_attribute.type_()
		{
			// Only if have a value.
			(false, false, IFA_TARGET_NETNSID) => set_field_error(&mut processing_message_state.common.target_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			
			// Mandatory.
			(false, false, IFA_CACHEINFO) => set_field_error(&mut processing_message_state.common.cache_information, message_attribute, |message_attribute| message_attribute.get_attribute_value_struct_cloned::<ifa_cacheinfo>())?,
			
			// Optional.
			(false, false, IFA_LOCAL) => panic!("Attribute should only be present for `RTM_GETADDR` for Internet Protocol version 6"),
			(false, false, IFA_ADDRESS) => panic!("Attribute should only be present for `RTM_GETADDR` for Internet Protocol version 6"),
			(false, false, IFA_RT_PRIORITY) => panic!("Attribute should only be present for `RTM_GETADDR` for Internet Protocol version 6"),
			
			// Mandatory.
			(false, false, IFA_FLAGS) => panic!("Attribute should only be present for `RTM_GETADDR` for Internet Protocol version 6"),
			
			// Optional.
			// IPv4 only.
			(false, false, IFA_LABEL) => panic!("Attribute should only be present for Internet Protocol version 4"),
			(false, false, IFA_BROADCAST) => panic!("Attribute should only be present for Internet Protocol version 4"),
			
			// Optional.
			// Other.
			(false, false, IFA_MULTICAST) => panic!("Attribute should only be present for `RTM_GETANYCAST` for Internet Protocol version 6"),
			(false, false, IFA_ANYCAST) => set_address_field(&mut processing_message_state.other_cast_address, message_attribute, rtattr::get_attribute_value_raw_protocol_address)?,
			
			(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
			
			(_, _, IFA_UNSPEC) => (),
			
			_ => (),
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn finalize(&self, processing_message_state: Self::ProcessingMessageState) -> Result<Self::ProcessedMessage, String>
	{
		processing_message_state.to_processed_message()
	}
}
