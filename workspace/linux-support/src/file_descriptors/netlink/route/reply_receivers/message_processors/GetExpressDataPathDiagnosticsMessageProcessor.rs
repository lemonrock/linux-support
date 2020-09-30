// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct GetExpressDataPathDiagnosticsMessageProcessor;

impl MessageProcessor for GetExpressDataPathDiagnosticsMessageProcessor
{
	type Header = xdp_diag_msg;
	
	type ProcessingMessageState = GetExpressDataPathDiagnosticsProcessingMessageState;
	
	type ProcessedMessage = GetExpressDataPathDiagnosticsMessageData;
	
	type NAT = XDP_DIAG;
	
	#[inline(always)]
	fn process_message_header(&self, message_header: &Self::Header) -> Result<Option<Self::ProcessingMessageState>, String>
	{
		GetExpressDataPathDiagnosticsProcessingMessageState::new(message_header).map(|processing_message_state| Some(processing_message_state))
	}
	
	#[inline(always)]
	fn process_message_attribute(&self, message_attribute: &rtattr<Self::NAT>, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>
	{
		use self::XDP_DIAG::*;
		
		match message_attribute.type_()
		{
			(false, false, XDP_DIAG_INFO) => set_field_error(&mut processing_message_state.basic_information, message_attribute, |message_attribute| message_attribute.get_attribute_value_struct_cloned::<xdp_diag_info>())?,
			
			(false, false, XDP_DIAG_UID) => set_field_error(&mut processing_message_state.user_identifier, message_attribute, |message_attribute| message_attribute.get_attribute_value_uid())?,
			
			(false, false, XDP_DIAG_RX_RING) => set_field_error(&mut processing_message_state.receive_ring_number_of_descriptors, message_attribute, |message_attribute| message_attribute.get_attribute_value_ring_number_of_descriptors())?,
			
			(false, false, XDP_DIAG_TX_RING) => set_field_error(&mut processing_message_state.transmit_ring_number_of_descriptors, message_attribute, |message_attribute| message_attribute.get_attribute_value_ring_number_of_descriptors())?,
			
			(false, false, XDP_DIAG_UMEM_FILL_RING) => set_field_error(&mut processing_message_state.fill_ring_number_of_descriptors, message_attribute, |message_attribute| message_attribute.get_attribute_value_ring_number_of_descriptors())?,
			
			(false, false, XDP_DIAG_UMEM_COMPLETION_RING) => set_field_error(&mut processing_message_state.completion_ring_number_of_descriptors, message_attribute, |message_attribute| message_attribute.get_attribute_value_ring_number_of_descriptors())?,
			
			(false, false, XDP_DIAG_UMEM) => set_field_error(&mut processing_message_state.user_memory_information, message_attribute, |message_attribute| message_attribute.get_attribute_value_struct_cloned::<xdp_diag_umem>())?,
			
			(false, false, XDP_DIAG_MEMINFO) => set_field_error(&mut processing_message_state.socket_memory_information, message_attribute, |message_attribute| message_attribute.socket_memory_information())?,
			
			(false, false, XDP_DIAG_STATS) => set_field_error(&mut processing_message_state.statistics, message_attribute, |message_attribute| message_attribute.get_attribute_value_struct_cloned::<xdp_diag_stats>())?,
			
			(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
			
			(_, _, XDP_DIAG_NONE) => (),
			
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

impl GetExpressDataPathDiagnosticsMessageProcessor
{
	#[inline(always)]
	pub(crate) fn get_diagnostics(&self, netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>) -> Result<Vec<GetExpressDataPathDiagnosticsMessageData>, String>
	{
		RouteNetlinkProtocol::make_request_and_get_reply_messages
		(
			netlink_socket_file_descriptor,
			self,
			Self::new_route_get_message()).map_err(|error| match error
			{
				Left(messaging_parsing_errors) => format!("Message parsing errors {:?}", messaging_parsing_errors),
			
				Right(end_of_set_of_messages_error) => format!("End of set of messages errors {}", end_of_set_of_messages_error),
			}
		)
	}
	
	#[inline(always)]
	fn new_route_get_message() -> NetlinkRequestMessage<xdp_diag_req>
	{
		const SOCK_DIAG_BY_FAMILY: RouteNetlinkMessageType = RouteNetlinkMessageType::NEWADDR;
		RouteNetlinkProtocol::new_get_request_message(SOCK_DIAG_BY_FAMILY, NetlinkGetRequestMessageFlags::Dump, xdp_diag_req::for_get())
	}
}
