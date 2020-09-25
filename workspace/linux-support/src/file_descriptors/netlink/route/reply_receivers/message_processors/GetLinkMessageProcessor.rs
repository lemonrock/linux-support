// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct GetLinkMessageProcessor;

impl MessageProcessor for GetLinkMessageProcessor
{
	type Header = ifinfomsg;
	
	type ProcessingMessageState = GetLinkProcessingMessageState;
	
	type ProcessedMessage = GetLinkMessageData;
	
	type NAT = IFLA;
	
	#[doc(hidden)]
	#[inline(always)]
	fn validate_message_type(message_type: RouteNetlinkMessageType) -> Result<(), String>
	{
		use self::RouteNetlinkMessageType::*;
		
		match message_type
		{
			NEWLINK | DELLINK => Ok(()),
			
			_ => Err(format!("Unexpeced message type `{:?}`", message_type)),
		}
	}
	
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
		
		use self::IFLA::*;
		
		match message_attribute.type_()
		{
			// Mandatory.
			(false, false, IFLA_IFNAME) => Self::set_field_error(&mut processing_message_state.network_interface_name, message_attribute, rtattr::get_attribute_value_network_interface_name)?,
			(false, false, IFLA_TXQLEN) => Self::set_field_error(&mut processing_message_state.transmission_queue_length, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_OPERSTATE) => Self::set_field_error(&mut processing_message_state.operational_status, message_attribute, rtattr::get_attribute_value_operational_status)?,
			(false, false, IFLA_LINKMODE) => Self::set_field_error(&mut processing_message_state.link_mode, message_attribute, rtattr::get_attribute_value_link_mode)?,
			(false, false, IFLA_MTU) => Self::set_field_error(&mut processing_message_state.maximum_transmission_unit, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_MIN_MTU) => Self::set_field_error(&mut processing_message_state.maximum_transmission_unit, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_MAX_MTU) => Self::set_field_error(&mut processing_message_state.maximum_transmission_unit_minimum, message_attribute, rtattr::get_attribute_value_maximum_transmission_unit)?,
			(false, false, IFLA_GROUP) => Self::set_field_error(&mut processing_message_state.group, message_attribute, rtattr::get_attribute_value_net_dev_group)?,
			(false, false, IFLA_PROMISCUITY) => Self::set_field_error(&mut processing_message_state.promiscuity, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_NUM_TX_QUEUES) => Self::set_field_error(&mut processing_message_state.number_of_transmission_queues, message_attribute, rtattr::get_attribute_value_queue_count)?,
			(false, false, IFLA_GSO_MAX_SEGS) => Self::set_field_error(&mut processing_message_state.generic_segmentation_offload_maximum_segments, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_GSO_MAX_SIZE) => Self::set_field_error(&mut processing_message_state.generic_segmentation_offload_maximum_size, message_attribute, rtattr::get_attribute_value_u32)?,
			
			// Only if Linux so configured.
			(false, false, IFLA_NUM_RX_QUEUES) => Self::set_field_error(&mut processing_message_state.number_of_receive_queues, message_attribute, rtattr::get_attribute_value_queue_count)?,
			
			// Optional.
			(false, false, IFLA_MASTER) => Self::set_field_error(&mut processing_message_state.master_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index)?,
			
			// Mandatory.
			(false, false, IFLA_CARRIER) => Self::set_field_error(&mut processing_message_state.carrier_ok, message_attribute, rtattr::get_attribute_value_bool)?,
			
			// Optional.
			(false, false, IFLA_QDISC) => Self::set_field_error(&mut processing_message_state.queueing_discipline, message_attribute, rtattr::get_attribute_value_queuing_discipline)?,
			(false, false, IFLA_IFALIAS) => Self::set_field_error(&mut processing_message_state.network_interface_alias, message_attribute, rtattr::get_attribute_value_network_interface_alias)?,
			
			// Mandatory.
			(false, false, IFLA_CARRIER_CHANGES) => Self::set_field_error(&mut processing_message_state.carrier_up_and_down_count, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_PROTO_DOWN) => Self::set_field_error(&mut processing_message_state.proto_down, message_attribute, rtattr::get_attribute_value_bool)?,
			
			// Nested and optional.
			// Only present if `IFLA_PROTO_DOWN` present, true and reason code is non-zero.
			(true, false, IFLA_PROTO_DOWN_REASON) => Self::set_field_error(&mut processing_message_state.protocol_down_reason_value, message_attribute, |message_attribute| Self::process_IFLA_PROTO_DOWN_REASON(message_attribute.get_attribute_value_nested()))?,
			
			// Mandatory.
			(false, false, IFLA_CARRIER_UP_COUNT) => Self::set_field_error(&mut processing_message_state.carrier_up_count, message_attribute, rtattr::get_attribute_value_u32)?,
			(false, false, IFLA_CARRIER_DOWN_COUNT) => Self::set_field_error(&mut processing_message_state.carrier_down_count, message_attribute, rtattr::get_attribute_value_u32)?,
			
			// Optional.
			(false, false, IFLA_TARGET_NETNSID) => Self::set_field_error(&mut processing_message_state.target_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			
			// Mandatory.
			(false, false, IFLA_MAP) => Self::set_field_error(&mut processing_message_state.map, message_attribute, rtattr::get_attribute_value_struct_cloned::<rtnl_link_ifmap>)?,
			
			// Optional but occur together.
			(false, false, IFLA_ADDRESS) => set_address_field(&mut processing_message_state.address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			(false, false, IFLA_BROADCAST) => set_address_field(&mut processing_message_state.broadcast_address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			(false, false, IFLA_PERM_ADDRESS) => set_address_field(&mut processing_message_state.permanent_address, message_attribute, rtattr::get_attribute_value_hardware_address)?,
			
			// Optional.
			(false, false, IFLA_PHYS_PORT_ID) => Self::set_field_error(&mut processing_message_state.physical_port_identifier, message_attribute, rtattr::get_attribute_value_physical_identifier)?,
			(false, false, IFLA_PHYS_PORT_NAME) => Self::set_field_error(&mut processing_message_state.physical_port_name, message_attribute, rtattr::get_attribute_value_c_string)?,
			(false, false, IFLA_PHYS_SWITCH_ID) => Self::set_field_error(&mut processing_message_state.physical_switch_identifier, message_attribute, rtattr::get_attribute_value_physical_identifier)?,
			
			(false, false, IFLA_STATS64) => Self::set_field_error(&mut processing_message_state.statistics, message_attribute, rtattr::get_attribute_value_struct_cloned::<rtnl_link_stats64>)?,
			
			// Legacy 32-bit version of `IFLA_STATS64`.
			(false, false, IFLA_STATS) => (),
			
			// Nested and optional.
			// rtnl_fill_vf IFLA_VF_PORTS for virtual functions.
			// rtnl_port_fill IFLA_PORT_SELF for virtual functions.
			// rtnl_link_fill IFLA_LINKINFO for details; not present for loopback, ethernet and WiFi devices. Contains `IFLA_INFO_KIND` with values such as `veth` and `bridge`.
			// rtnl_fill_link_af IFLA_AF_SPEC for address families.
			// rtnl_fill_prop_list IFLA_PROP_LIST for list of repeated IFLA_ALT_IFNAME interface alternative names.
			(true, false, IFLA_XDP) => Self::set_field_error(&mut processing_message_state.express_data_path, message_attribute, |message_attribute| Self::process_IFLA_XDP(message_attribute.get_attribute_value_nested()))?,
			
			(false, false, IFLA_LINK_NETNSID) => Self::set_field_error(&mut processing_message_state.linked_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			(false, false, IFLA_LINK) => Self::set_field_error(&mut processing_message_state.linked_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index.map(Some))?,
			(false, false, IFLA_NEW_NETNSID) => Self::set_field_error(&mut processing_message_state.new_net_namespace_identifier, message_attribute, rtattr::get_attribute_value_net_namespace_identifier)?,
			(false, false, IFLA_NEW_IFINDEX) => Self::set_field_error(&mut processing_message_state.new_network_interface_index, message_attribute, rtattr::get_attribute_value_network_interface_index)?,
			
			(true, false, IFLA_PROP_LIST) => Self::set_field_error(&mut processing_message_state.alternative_network_interface_names, message_attribute, |message_attribute| Self::process_IFLA_PROP_LIST(message_attribute.get_attribute_value_nested()))?,
			
			(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
			
			// A non-zero u32; only when events are sent using RTM_NEWLINK.
			(false, false, IFLA_EVENT) => (),
			
			// Only when events are sent using RTM_NEWLINK; variable length binary data.
			(false, false, IFLA_WIRELESS) => (),
			
			// Should not be present; used internally when reserving space by `rtnetlink.c` for `IFLA_STATS64` and `IFLA_MAP`.
			(false, false, IFLA_PAD) => (),
			
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
	
	fn process_IFLA_PROTO_DOWN_REASON(nested_attributes: &[u8]) -> Result<NonZeroU32, String>
	{
		let mut protocol_down_reason_value: Option<NonZeroU32> = None;
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA_PROTO_DOWN_REASON>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IFLA_PROTO_DOWN_REASON::*;
			match message_attribute.type_()
			{
				(false, false, IFLA_PROTO_DOWN_REASON_VALUE) =>
				{
					if unlikely!(protocol_down_reason_value.is_some())
					{
						return Err(format!("IFLA_PROTO_DOWN_REASON_VALUE can only be specified once"))
					}
					protocol_down_reason_value = Some(message_attribute.get_attribute_value_non_zero_u32()?);
				}
				
				(false, false, IFLA_PROTO_DOWN_REASON_MASK) => return Err(format!("IFLA_PROTO_DOWN_REASON_MASK should not be present")),
			
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, IFLA_PROTO_DOWN_REASON_UNSPEC) => (),
				
				_ => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		protocol_down_reason_value.ok_or(format!("IFLA_PROTO_DOWN_REASON_VALUE must be specified"))
	}
	
	fn process_IFLA_PROP_LIST(nested_attributes: &[u8]) -> Result<Vec<NetworkInterfaceAlternativeName>, String>
	{
		let mut alternative_interface_names = Vec::new();
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IFLA::*;
			match message_attribute.type_()
			{
				(false, false, IFLA_ALT_IFNAME) =>
				{
					let network_interface_alternative_name = message_attribute.get_attribute_value_network_interface_alternative_name()?;
					alternative_interface_names.push(network_interface_alternative_name);
				}
				
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, IFLA_UNSPEC) => (),
				
				// Other attributes in the property list (not as of Linux 5.7).
				(_, _, _) => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		Ok(alternative_interface_names)
	}
	
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
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
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
					message_data.attached = message_attribute.get_attribute_value_attached().map_err(|error| format!("{}", error))?;
					attached = true;
				}
				
				(false, false, IFLA_XDP_PROG_ID) => set_program_identifier_field(&mut message_data.program_identifier, message_attribute)?,
				
				(false, false, IFLA_XDP_SKB_PROG_ID) => set_program_identifier_field(&mut message_data.generic_program_identifier, message_attribute)?,
				
				(false, false, IFLA_XDP_DRV_PROG_ID) => set_program_identifier_field(&mut message_data.native_program_identifier, message_attribute)?,
				
				(false, false, IFLA_XDP_HW_PROG_ID) => set_program_identifier_field(&mut message_data.offloaded_program_identifier, message_attribute)?,
			
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
	
	fn process_IFLA_AF_SPEC(nested_attributes: &[u8]) -> Result<(Option<InternetProtocolVersion4Details>, Option<InternetProtocolVersion6Details>), String>
	{
		let mut internet_protocol_version_4_device_configuration: Option<InternetProtocolVersion4DeviceConfiguration> = None;
		let mut internet_protocol_version_6_details: Option<InternetProtocolVersion6Details> = None;
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA_AF_SPEC>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use IFLA_AF_SPEC::*;
			match message_attribute.type_()
			{
				(true, false, IFLA_AF_SPEC_INET) =>
				{
					if unlikely!(internet_protocol_version_4_device_configuration.is_some())
					{
						return Err(format!("IFLA_AF_SPEC_INET can only be specified once"))
					}
					
					internet_protocol_version_4_device_configuration = Some(Self::process_IFLA_AF_SPEC_INET(message_attribute.get_attribute_value_nested())?);
				},
				
				(true, false, IFLA_AF_SPEC_INET6) =>
				{
					if unlikely!(internet_protocol_version_6_details.is_some())
					{
						return Err(format!("IFLA_AF_SPEC_INET6 can only be specified once"))
					}
					
					internet_protocol_version_6_details = Some(Self::process_IFLA_AF_SPEC_INET6(message_attribute.get_attribute_value_nested())?);
				},
				
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, _) => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		Ok
		(
			(
				internet_protocol_version_4_device_configuration.map(|device_configuration| InternetProtocolVersion4Details { device_configuration }),
				internet_protocol_version_6_details,
			)
		)
	}
	
	fn process_IFLA_AF_SPEC_INET(nested_attributes: &[u8]) -> Result<InternetProtocolVersion4DeviceConfiguration, String>
	{
		let mut internet_protocol_version_4_device_configuration: Option<InternetProtocolVersion4DeviceConfiguration> = None;
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA_INET>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IFLA_INET::*;
			match message_attribute.type_()
			{
				(true, false, IFLA_INET_CONF) =>
				{
					if unlikely!(internet_protocol_version_4_device_configuration.is_some())
					{
						return Err(format!("IFLA_INET_CONF can only be specified once"))
					}
					internet_protocol_version_4_device_configuration = Some(Self::process_IFLA_INET_CONF(message_attribute.get_attribute_value_nested())?);
				},
			
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, IFLA_INET_UNSPEC) => (),
				
				_ => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		internet_protocol_version_4_device_configuration.ok_or(format!("IFLA_INET_CONF has not been specified once"))
	}
	
	fn process_IFLA_INET_CONF(nested_attributes: &[u8]) -> Result<InternetProtocolVersion4DeviceConfiguration, String>
	{
		let mut processing_message_state = InternetProtocolVersion4DeviceConfigurationGetLinkProcessMessageState::default();
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IPV4_DEVCONF>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IPV4_DEVCONF::*;
			match message_attribute.type_()
			{
				(false, false, IPV4_DEVCONF_FORWARDING) => Self::set_field_error(&mut processing_message_state.forwarding, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, IPV4_DEVCONF_MC_FORWARDING) => Self::set_field_error(&mut processing_message_state.multicast_forwarding, message_attribute, rtattr::get_attribute_value_bool)?,
			
				(false, false, IPV4_DEVCONF_PROXY_ARP) => Self::set_field_error(&mut processing_message_state.proxy_arp, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ACCEPT_REDIRECTS) => Self::set_field_error(&mut processing_message_state.accept_redirects, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_SECURE_REDIRECTS) => Self::set_field_error(&mut processing_message_state.secure_redirects, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_SEND_REDIRECTS) => Self::set_field_error(&mut processing_message_state.send_redirects, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_SHARED_MEDIA) => Self::set_field_error(&mut processing_message_state.shared_media, message_attribute, rtattr::get_attribute_value_u32)?,

				(false, false, IPV4_DEVCONF_RP_FILTER) => Self::set_field_error(&mut processing_message_state.reverse_path_filter, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ACCEPT_SOURCE_ROUTE) => Self::set_field_error(&mut processing_message_state.accept_source_route, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_BOOTP_RELAY) => Self::set_field_error(&mut processing_message_state.bootp_relay, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_LOG_MARTIANS) => Self::set_field_error(&mut processing_message_state.log_martians, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_TAG) => Self::set_field_error(&mut processing_message_state.tag, message_attribute, rtattr::get_attribute_value_u32)?,

				(false, false, IPV4_DEVCONF_ARPFILTER) => Self::set_field_error(&mut processing_message_state.arp_filter, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_MEDIUM_ID) => Self::set_field_error(&mut processing_message_state.medium_id, message_attribute, rtattr::get_attribute_value_u32)?,

				(false, false, IPV4_DEVCONF_NOXFRM) => Self::set_field_error(&mut processing_message_state.disable_xfrm, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_NOPOLICY) => Self::set_field_error(&mut processing_message_state.disable_policy, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_FORCE_IGMP_VERSION) => Self::set_field_error(&mut processing_message_state.force_igmp_version, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ARP_ANNOUNCE) => Self::set_field_error(&mut processing_message_state.arp_announce, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ARP_IGNORE) => Self::set_field_error(&mut processing_message_state.arp_ignore, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_PROMOTE_SECONDARIES) => Self::set_field_error(&mut processing_message_state.promote_secondaries, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ARP_ACCEPT) => Self::set_field_error(&mut processing_message_state.arp_accept, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ARP_NOTIFY) => Self::set_field_error(&mut processing_message_state.arp_notify, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ACCEPT_LOCAL) => Self::set_field_error(&mut processing_message_state.accept_local, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_SRC_VMARK) => Self::set_field_error(&mut processing_message_state.source_valid_mark, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_PROXY_ARP_PVLAN) => Self::set_field_error(&mut processing_message_state.proxy_arp_pvlan, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_ROUTE_LOCALNET) => Self::set_field_error(&mut processing_message_state.route_localnet, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_IGMPV2_UNSOLICITED_REPORT_INTERVAL) => Self::set_field_error(&mut processing_message_state.igmpv2_unsolicited_report_interval, message_attribute, rtattr::get_attribute_value_u32)?,

				(false, false, IPV4_DEVCONF_IGMPV3_UNSOLICITED_REPORT_INTERVAL) => Self::set_field_error(&mut processing_message_state.igmpv3_unsolicited_report_interval, message_attribute, rtattr::get_attribute_value_u32)?,

				(false, false, IPV4_DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN) => Self::set_field_error(&mut processing_message_state.ignore_routes_with_link_down, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_DROP_UNICAST_IN_L2_MULTICAST) => Self::set_field_error(&mut processing_message_state.drop_unicast_in_layer2_multicast, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_DROP_GRATUITOUS_ARP) => Self::set_field_error(&mut processing_message_state.drop_gratuitous_arp, message_attribute, rtattr::get_attribute_value_bool)?,

				(false, false, IPV4_DEVCONF_BC_FORWARDING) => Self::set_field_error(&mut processing_message_state.broadcast_forwarding, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				_ => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		processing_message_state.to_processed_message()
	}
	
	// See `inet6_fill_ifla6_attrs()`.
	fn process_IFLA_AF_SPEC_INET6(nested_attributes: &[u8]) -> Result<InternetProtocolVersion6Details, String>
	{
		let mut processing_message_state = InternetProtocolVersion6DetailsGetLinkProcessMessageState::default();
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<IFLA_INET6>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { & * potential_message_attribute_pointer };
			
			use self::IFLA_INET6::*;
			match message_attribute.type_()
			{
				(false, false, IFLA_INET6_FLAGS) => Self::set_field_error(&mut processing_message_state.flags, message_attribute, rtattr::get_attribute_value_u32),
				
				(true, false, IFLA_INET6_CONF) => Self::set_field_error(&mut processing_message_state.device_configuration, message_attribute, |message_attribute| Self::process_IFLA_INET6_CONF(message_attribute.get_attribute_value_nested())),
				
				(false, false, IFLA_INET6_STATS) => Self::set_field_error(&mut processing_message_state.statistics, message_attribute, rtattr::get_inet6_statistics),
				
				// IFLA_INET6_MCAST - unsupported
				
				(false, false, IFLA_INET6_CACHEINFO) => Self::set_field_error(&mut processing_message_state.cache_information, message_attribute, rtattr::get_attribute_value_struct_cloned),
				
				(false, false, IFLA_INET6_ICMP6STATS) => Self::set_field_error(&mut processing_message_state.icmpv6_statistics, message_attribute, rtattr::get_inet6_statistics),
				
				(false, false, IFLA_INET6_TOKEN) => Self::set_field_error(&mut processing_message_state.token, message_attribute, rtattr::get_attribute_value_struct_cloned),
				
				(false, false, IFLA_INET6_ADDR_GEN_MODE) => Self::set_field_error(&mut processing_message_state.address_generation_mode, message_attribute, rtattr::get_attribute_value_address_generation_mode),
				
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, IFLA_INET6_UNSPEC) => (),
				
				_ => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		processing_message_state.to_processed_message()
	}
	
	fn process_IFLA_INET6_CONF(nested_attributes: &[u8]) -> Result<InternetProtocolVersion6DeviceConfiguration, String>
	{
		let mut processing_message_state = InternetProtocolVersion6DeviceConfigurationGetLinkProcessMessageState::default();
		
		let pointer = nested_attributes.as_ptr();
		let mut potential_message_attribute_pointer = pointer as *const rtattr<DEVCONF>;
		let end = unsafe { potential_message_attribute_pointer.add(nested_attributes.len()) };
		while rtattr::ok(potential_message_attribute_pointer, end)
		{
			let message_attribute = unsafe { &*potential_message_attribute_pointer };
			
			use self::DEVCONF::*;
			match message_attribute.type_()
			{
				(false, false, DEVCONF_FORWARDING) => Self::set_field_error(&mut processing_message_state.forwarding, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_HOPLIMIT) => Self::set_field_error(&mut processing_message_state.hop_limit, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_MTU6) => Self::set_field_error(&mut processing_message_state.maximum_transmission_unit6, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_ACCEPT_RA) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_REDIRECTS) => Self::set_field_error(&mut processing_message_state.accept_redirects, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_AUTOCONF) => Self::set_field_error(&mut processing_message_state.autoconf, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_DAD_TRANSMITS) => Self::set_field_error(&mut processing_message_state.duplicate_address_detection_transmits, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_RTR_SOLICITS) => Self::set_field_error(&mut processing_message_state.router_solicits, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_RTR_SOLICIT_INTERVAL) => Self::set_field_error(&mut processing_message_state.router_solicit_interval, message_attribute, rtattr::get_attribute_value_millisconds)?,
				
				(false, false, DEVCONF_RTR_SOLICIT_DELAY) => Self::set_field_error(&mut processing_message_state.router_solicit_delay, message_attribute, rtattr::get_attribute_value_millisconds)?,
				
				(false, false, DEVCONF_USE_TEMPADDR) => Self::set_field_error(&mut processing_message_state.use_temporary_address, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_TEMP_VALID_LFT) => Self::set_field_error(&mut processing_message_state.temporary_address_valid_lifetime, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_TEMP_PREFERED_LFT) => Self::set_field_error(&mut processing_message_state.temporary_address_prefered_lifetime, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_REGEN_MAX_RETRY) => Self::set_field_error(&mut processing_message_state.regen_maximum_retry, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_MAX_DESYNC_FACTOR) => Self::set_field_error(&mut processing_message_state.maximum_desync_factor, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_MAX_ADDRESSES) => Self::set_field_error(&mut processing_message_state.maximum_addresses, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_FORCE_MLD_VERSION) => Self::set_field_error(&mut processing_message_state.force_multicast_listener_discovery_version, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_DEFRTR) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_default_router, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_PINFO) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_prefix_information, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_RTR_PREF) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_router_preference, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_RTR_PROBE_INTERVAL) => Self::set_field_error(&mut processing_message_state.router_probe_interval, message_attribute, rtattr::get_attribute_value_milliseconds)?,
				
				(false, false, DEVCONF_ACCEPT_RA_RT_INFO_MAX_PLEN) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_route_information_maximum_prefix_length, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_PROXY_NDP) => Self::set_field_error(&mut processing_message_state.proxy_neighbor_discovery_protocol, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_OPTIMISTIC_DAD) => Self::set_field_error(&mut processing_message_state.optimistic_duplicate_address_detection, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_SOURCE_ROUTE) => Self::set_field_error(&mut processing_message_state.accept_source_route, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_MC_FORWARDING) => Self::set_field_error(&mut processing_message_state.mulitcast_forwarding, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_DISABLE_IPV6) => Self::set_field_error(&mut processing_message_state.disable_ipv6, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_DAD) => Self::set_field_error(&mut processing_message_state.accept_duplicate_address_detection, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_FORCE_TLLAO) => Self::set_field_error(&mut processing_message_state.force_force_target_link_layer_address_option, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_NDISC_NOTIFY) => Self::set_field_error(&mut processing_message_state.icmpv6_neighbor_discovery_notify, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_MLDV1_UNSOLICITED_REPORT_INTERVAL) => Self::set_field_error(&mut processing_message_state.multicast_listener_discovery_v1_unsolicited_report_interval, message_attribute, rtattr::get_attribute_value_milliseconds)?,
				
				(false, false, DEVCONF_MLDV2_UNSOLICITED_REPORT_INTERVAL) => Self::set_field_error(&mut processing_message_state.multicast_listener_discovery_v2_unsolicited_report_interval, message_attribute, rtattr::get_attribute_value_milliseconds)?,
				
				(false, false, DEVCONF_SUPPRESS_FRAG_NDISC) => Self::set_field_error(&mut processing_message_state.icmpv6_neighbor_discovery_suppress_fragments, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_FROM_LOCAL) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_from_local, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_USE_OPTIMISTIC) => Self::set_field_error(&mut processing_message_state.use_optimistic_duplicate_address_detection, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_MTU) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_maximum_transmission_unit, message_attribute, rtattr::get_attribute_value_bool)?,
				
				// Not currently reported.
				(false, false, DEVCONF_STABLE_SECRET) => (),
				
				(false, false, DEVCONF_USE_OIF_ADDRS_ONLY) => Self::set_field_error(&mut processing_message_state.use_output_interface_addresses_only, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_MIN_HOP_LIMIT) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_minimum_hop_limit, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_IGNORE_ROUTES_WITH_LINKDOWN) => Self::set_field_error(&mut processing_message_state.ignore_routes_with_link_down, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_DROP_UNICAST_IN_L2_MULTICAST) => Self::set_field_error(&mut processing_message_state.drop_unicast_in_layer2_multicast, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_DROP_UNSOLICITED_NA) => Self::set_field_error(&mut processing_message_state.drop_unsolicited_neighbor_advertisements, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_KEEP_ADDR_ON_DOWN) => Self::set_field_error(&mut processing_message_state.keep_address_on_down, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_RTR_SOLICIT_MAX_INTERVAL) => Self::set_field_error(&mut processing_message_state.router_solicit_maximum_interval, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_SEG6_ENABLED) => Self::set_field_error(&mut processing_message_state.seg6_enabled, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_SEG6_REQUIRE_HMAC) => Self::set_field_error(&mut processing_message_state.seg6_require_hmac, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ENHANCED_DAD) => Self::set_field_error(&mut processing_message_state.enhanced_duplicate_address_detection, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ADDR_GEN_MODE) => Self::set_field_error(&mut processing_message_state.address_generation_mode, message_attribute, rtattr::get_attribute_value_ENUMMMMMM)?,
				
				(false, false, DEVCONF_DISABLE_POLICY) => Self::set_field_error(&mut processing_message_state.disable_policy, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(false, false, DEVCONF_ACCEPT_RA_RT_INFO_MIN_PLEN) => Self::set_field_error(&mut processing_message_state.accept_router_advertisement_route_information_minimum_prefix_length, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_NDISC_TCLASS) => Self::set_field_error(&mut processing_message_state.icmpv6_neighbor_discovery_traffic_class, message_attribute, rtattr::get_attribute_value_u32)?,
				
				(false, false, DEVCONF_RPL_SEG_ENABLED) => Self::set_field_error(&mut processing_message_state.rpl_seg_enabled, message_attribute, rtattr::get_attribute_value_bool)?,
				
				(true, true, _) => panic!("Attribute may not be both nested and in network byte order"),
				
				(_, _, _) => (),
			}
			
			potential_message_attribute_pointer = message_attribute.next()
		}
		
		processing_message_state.to_processed_message()
	}
	
	#[inline(always)]
	fn set_field_error<Field, Error: ToString, NAT: NetlinkAttributeType, F: FnOnce(&rtattr<NAT>) -> Result<Field, Error>>(field: &mut Option<Field>, message_attribute: &rtattr<NAT>, attribute: F) -> Result<(), String>
	{
		*field.as_mut().ok_or(format!("field already populated; duplicate rtattr"))? = attribute(message_attribute).map_err(|error| error.to_string())?;
		Ok(())
	}
}
