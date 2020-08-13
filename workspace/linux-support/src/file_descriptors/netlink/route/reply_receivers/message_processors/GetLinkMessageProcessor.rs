// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct GetLinkMessageProcessor;

impl MessageProcessor for GetLinkMessageProcessor
{
	type Header = ifinfomsg;
	
	type ProcessingMessageState = GetLinkProcessingMessageState;
	
	type ProcessedMessage = GetLinkMessageData;
	
	type NAT = IFLA;
	
	#[inline(always)]
	fn process_message_header(&self, message_header: &Self::Header) -> Result<Option<Self::ProcessingMessageState>, String>
	{
		GetLinkProcessingMessageState::new(message_header).map(|processing_message_state| Some(processing_message_state))
	}
	
	#[inline(always)]
	fn process_message_attribute(&self, message_attribute: &rtattr<Self::NAT>, processing_message_state: &mut Self::ProcessingMessageState) -> Result<(), String>
	{
		#[inline(always)]
		fn set_address_field<F: FnOnce(&rtattr<IFLA>) -> &[u8]>(field: &mut Option<HardwareAddress>, message_attribute: &rtattr<IFLA>, attribute: F) -> Result<(), String>
		{
			set_field_error(field, message_attribute, |message_attribute| HardwareAddress::try_from(attribute(message_attribute)))
		}
		
		#[inline(always)]
		fn set_field_error<Field, Error: ToString, F: FnOnce(&rtattr<IFLA>) -> Result<Field, Error>>(field: &mut Option<Field>, message_attribute: &rtattr<IFLA>, attribute: F) -> Result<(), String>
		{
			*field.as_mut().ok_or(format!("field already populated; duplicate rtattr"))? = attribute(message_attribute).map_err(|error| error.to_string())?;
			Ok(())
		}
		
		use self::IFLA::*;
		
		match message_attribute.type_()
		{
			// Mandatory.
			(false, false, IFLA_IFNAME) => set_field_error(&mut processing_message_state.network_interface_name, message_attribute, rtattr::get_attribute_value_network_interface_name)?,
			(false, false, IFLA_TXQLEN) => set_field_error(&mut processing_message_state.transmission_queue_length, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_OPERSTATE) => set_field_error(&mut processing_message_state.operational_status, message_attribute, rtattr::get_attribute_value_operational_status)?,
			(false, false, IFLA_LINKMODE) => set_field_error(&mut processing_message_state.link_mode, message_attribute, rtattr::get_attribute_value_link_mode)?,
			(false, false, IFLA_MTU) => set_field_error(&mut processing_message_state.maximum_transmission_unit, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_MIN_MTU) => set_field_error(&mut processing_message_state.maximum_transmission_unit, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_MAX_MTU) => set_field_error(&mut processing_message_state.maximum_transmission_unit_minimum, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_GROUP) => set_field_error(&mut processing_message_state.group, message_attribute, rtattr::get_attribute_value_net_dev_group)?,
			(false, false, IFLA_PROMISCUITY) => set_field_error(&mut processing_message_state.promiscuity, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_NUM_TX_QUEUES) => set_field_error(&mut processing_message_state.number_of_transmission_queues, message_attribute, rtattr::get_attribute_value_queue_count)?,
			(false, false, IFLA_GSO_MAX_SEGS) => set_field_error(&mut processing_message_state.generic_segmentation_offload_maximum_segments, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_GSO_MAX_SIZE) => set_field_error(&mut processing_message_state.generic_segmentation_offload_maximum_size, message_attribute, rtattr::get_attribute_value_u32)?,
			
			// Only if Linux so configured.
			(false, false, IFLA_NUM_RX_QUEUES) => set_field_error(&mut processing_message_state.number_of_receive_queues, message_attribute, rtattr::get_attribute_value_queue_count)?,
			
			// Optional.
			(false, false, IFLA_MASTER) => set_field_error(&mut processing_message_state.master_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index)?,
			
			// Mandatory.
			(false, false, IFLA_CARRIER) => set_field_error(&mut processing_message_state.carrier_ok, message_attribute, rtattr::get_attribute_value_bool)?,
			
			// Optional.
			(false, false, IFLA_QDISC) => set_field_error(&mut processing_message_state.queueing_discipline, message_attribute, rtattr::get_attribute_value_asciiz_string)?,
			(false, false, IFLA_IFALIAS) => set_field_error(&mut processing_message_state.network_interface_alias, message_attribute, rtattr::get_attribute_value_network_interface_alias)?,
			
			// Mandatory.
			(false, false, IFLA_CARRIER_CHANGES) => set_field_error(&mut processing_message_state.carrier_up_and_down_count, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_PROTO_DOWN) => set_field_error(&mut processing_message_state.proto_down, message_attribute, rtattr::get_attribute_value_bool)?,
			(false, false, IFLA_CARRIER_UP_COUNT) => set_field_error(&mut processing_message_state.carrier_up_count, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_CARRIER_DOWN_COUNT) => set_field_error(&mut processing_message_state.carrier_down_count, message_attribute, rtattr::get_attribute_value_u32)?,
			
			// Optional.
			(false, false, IFLA::IFLA_TARGET_NETNSID) => set_field_error(&mut processing_message_state.target_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			(false, false, IFLA_EVENT) => set_field_error(&mut processing_message_state.event, message_attribute, rtattr::get_attribute_value_non_zero_u32)?,
			
			// Mandatory.
			(false, false, IFLA_MAP) => set_field_error(&mut processing_message_state.map, message_attribute, rtattr::get_attribute_value_struct_cloned::<rtnl_link_ifmap>)?,
			
			// Optional but occur together.
			(false, false, IFLA_ADDRESS) => set_address_field(&mut processing_message_state.address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			(false, false, IFLA_BROADCAST) => set_address_field(&mut processing_message_state.broadcast_address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			(false, false, IFLA_PERM_ADDRESS) => set_address_field(&mut processing_message_state.permanent_address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			
			// Optional.
			(false, false, IFLA_PHYS_PORT_ID) => set_field_error(&mut processing_message_state.physical_port_identifier, message_attribute, rtattr::get_attribute_value_physical_identifier)?,
			(false, false, IFLA_PHYS_PORT_NAME) => set_field_error(&mut processing_message_state.physical_port_name, message_attribute, rtattr::get_attribute_value_asciiz_string)?,
			(false, false, IFLA_PHYS_SWITCH_ID) => set_field_error(&mut processing_message_state.physical_switch_identifier, message_attribute, rtattr::get_attribute_value_physical_identifier)?,
			
			(false, false, IFLA_STATS64) => set_field_error(&mut processing_message_state.statistics, message_attribute, rtattr::get_attribute_value_struct_cloned::<rtnl_link_stats64>)?,
			
			// Nested and optional.
			// rtnl_fill_vf IFLA_VF_PORTS for virtual functions.
			// rtnl_port_fill IFLA_PORT_SELF for virtual functions.
			// rtnl_link_fill IFLA_LINKINFO for details; not present for loopback, ethernet and WiFi devices. Contains `IFLA_INFO_KIND` with values such as `veth` and `bridge`.
			// rtnl_fill_link_af IFLA_AF_SPEC for address families.
			// rtnl_fill_prop_list IFLA_PROP_LIST for list of repeated IFLA_ALT_IFNAME interface alternative names.
			(true, false, IFLA_XDP) => set_field_error(&mut processing_message_state.express_data_path, message_attribute, |message_attribute| Self::process_IFLA_XDP(message_attribute.get_attribute_value_nested())),
			
			(false, false, IFLA_LINK_NETNSID) => set_field_error(&mut processing_message_state.linked_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			(false, false, IFLA_LINK) => set_field_error(&mut processing_message_state.linked_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index)?,
			(false, false, IFLA_NEW_NETNSID) => set_field_error(&mut processing_message_state.new_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			(false, false, IFLA_NEW_IFINDEX) => set_field_error(&mut processing_message_state.new_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index)?,
			
			(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
			
			(_, _, IFLA_STATS) => (),
			(_, _, IFLA_UNSPEC) => (),
			
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

impl GetLinkMessageProcessor
{
	fn process_IFLA_XDP(nested_attributes: &[u8]) -> Result<ExpressDataPathGetLinkMessageData, String>
	{
		#[inline(always)]
		fn set_program_identifier_field(field: &mut Option<ExtendedBpfProgramIdentifier>, message_attribute: &rtattr<IFLA_XDP>) -> Result<(), String>
		{
			*field.as_mut().ok_or(format!("field already populated; duplicate rtattr"))? = message_attribute.get_attribute_value_program_identifier().map_err(|error| error.to_string())?;
			Ok(())
		}
		
		let mut message_data = ExpressDataPathGetLinkMessageData::new();
		let mut attached = false;
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA_XDP>;
		let mut end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IFLA_XDP::*;
			match message_attribute.type_()
			{
				(false, false, IFLA_XDP_ATTACHED) =>
				{
					if unlikely!(attached)
					{
						return Err(format!("IFLA_XDP_ATTACHED can only be specified once"))
					}
					message_data.attached = message_attribute.get_attribute_value_attached()?;
					attached = true;
				}
				
				(false, false, IFLA_XDP_PROG_ID) => set_program_identifier_field(&mut message_data.program_identifier, message_attribute),
				
				(false, false, IFLA_XDP_SKB_PROG_ID) => set_program_identifier_field(&mut message_data.generic_program_identifier, message_attribute),
				
				(false, false, IFLA_XDP_DRV_PROG_ID) => set_program_identifier_field(&mut message_data.native_program_identifier, message_attribute),
				
				(false, false, IFLA_XDP_HW_PROG_ID) => set_program_identifier_field(&mut message_data.offloaded_program_identifier, message_attribute),
			
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, IFLA_XDP_UNSPEC) => (),
				
				_ => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		if unlikely!(!attached)
		{
			return Err(format!("IFLA_XDP_ATTACHED has not been specified once"))
		}
		Ok(message_data)
	}
	
	#[inline(always)]
	pub(crate) fn get_links(&self, netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>) -> Result<Vec<GetLinkMessageData>, String>
	{
		RouteNetlinkProtocol::make_request_and_get_reply_messages
		(
			netlink_socket_file_descriptor,
			self,
			Self::new_route_get_link_message()).map_err(|error| match error
			{
				Left(messaging_parsing_errors) => format!("Message parsing errors {:?}", messaging_parsing_errors),
			
				Right(end_of_set_of_messages_error) => format!("End of set of messages errors {}", end_of_set_of_messages_error),
			}
		)
	}
	
	#[inline(always)]
	fn new_route_get_link_message() -> NetlinkRequestMessage<ifinfomsg>
	{
		let body = ifinfomsg::for_get_link();
		RouteNetlinkProtocol::new_get_request_message(RouteNetlinkMessageType::GETLINK, NetlinkGetRequestMessageFlags::Dump, body)
	}
}
