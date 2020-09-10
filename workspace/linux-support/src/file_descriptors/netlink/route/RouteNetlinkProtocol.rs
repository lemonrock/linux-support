// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The `NETLINK_ROUTE` protocol.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RouteNetlinkProtocol;

impl NetlinkProtocol for RouteNetlinkProtocol
{
	type MessageType = RouteNetlinkMessageType;
	
	const Protocol: c_int = NETLINK_ROUTE;
	
	#[inline(always)]
	fn message_type(message_type: NetlinkMessageType) -> Self::MessageType
	{
		unsafe { message_type.route }
	}
	
	#[inline(always)]
	fn new_acknowledge_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkAcknowledgeMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_acknowledge_message(NetlinkMessageType { route: message_type }, flags, body)
	}
	
	#[inline(always)]
	fn new_delete_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkDeleteRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_delete_request_message(NetlinkMessageType { route: message_type }, flags, body)
	}
	
	#[inline(always)]
	fn new_get_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkGetRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_get_request_message(NetlinkMessageType { route: message_type }, flags, body)
	}
	
	#[inline(always)]
	fn new_set_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkSetRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_set_request_message(NetlinkMessageType { route: message_type }, flags, body)
	}
	
	#[inline(always)]
	fn new_new_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkNewRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_new_request_message(NetlinkMessageType { route: message_type }, flags, body)
	}
}

impl RouteNetlinkProtocol
{
	#[inline(always)]
	fn make_request_and_get_reply_messages<NRMB: NetlinkRequestMessageBody, RMP: MessageProcessor>(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>, route_message_processor: &RMP, mut request: NetlinkRequestMessage<NRMB>) -> Result<Vec<RMP::ProcessedMessage>, Either<Vec<String>, Errno>>
	{
		loop
		{
			let sequence_number = netlink_socket_file_descriptor.send_request(&mut request).expect("Send a request");
			
			let message_identification = MultipartMessagePartIdentification::from_linux_kernel(sequence_number);
			
			match RouteReplyReceiver::try_receiving_until_get_reply(netlink_socket_file_descriptor, route_message_processor, message_identification)
			{
				Ok(None) => continue,
				
				Ok(Some(processed_messages)) => return Ok(processed_messages),
				
				Err(error) => return Err(error),
			}
		}
	}
}

/// Get links.
impl RouteNetlinkProtocol
{
	/// Get link.
	pub fn get_link(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>, filter: &impl FnOnce(&GetLinkMessageData) -> bool) -> Result<Option<GetLinkMessageData>, String>
	{
		for get_link_message_data in Self::get_links(netlink_socket_file_descriptor)?
		{
			if filter(&get_link_message_data)
			{
				return Ok(Some(get_link_message_data))
			}
		}
		Ok(None)
	}
	
	/// Get links.
	///
	/// This is ***SLOW***.
	pub fn get_links(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>) -> Result<Vec<GetLinkMessageData>, String>
	{
		GetLinkMessageProcessor.get_links(netlink_socket_file_descriptor)
	}
}

/// Get addresses.
impl RouteNetlinkProtocol
{
	/// Get Internet Protocol version 4 addresses.
	///
	/// This is ***SLOW***.
	pub fn get_internet_protocol_version_4_addresses(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>) -> Result<Vec<GetAddressMessageData<in_addr>>, String>
	{
		GetAddressMessageProcessor::new().get_internet_protocol_addresses(netlink_socket_file_descriptor)
	}
	
	/// Get Internet Protocol version 6 addresses.
	///
	/// This is ***SLOW***.
	pub fn get_internet_protocol_version_6_addresses(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>) -> Result<Vec<GetAddressMessageData<in6_addr>>, String>
	{
		GetAddressMessageProcessor::new().get_internet_protocol_addresses(netlink_socket_file_descriptor)
	}
}

/// eXpress Data Path (XDP).
impl RouteNetlinkProtocol
{
	/// Attach a eXpress Data Path (XDP) program.
	///
	/// returns `ENODEV` if interface does not exist.
	///
	/// NOTE: As an alternative to using a network interface index, one can specify the top-level attribute `IFLA_IFNAME` with a network interface name.
	/// This is not supported by this functionality.
	///
	/// Based on the function `bpf_set_link_xdp_fd()` and its children in the Linux source `tools/lib/bpf/netlink.c`.
	pub fn xdp_fd_replace(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>, network_interface_index: NetworkInterfaceIndex, xdp_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, attach_mode: AttachMode, update_mode: UpdateMode) -> Result<(), Errno>
	{
		use self::IFLA_XDP::*;
		
		#[inline(always)]
		fn request_0(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>, network_interface_index: NetworkInterfaceIndex, xdp_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor) -> Result<(), Errno>
		{
			ExpressDataPathMessageBody::make_request_and_get_acknowledgment_or_error
			(
				netlink_socket_file_descriptor,
				network_interface_index,
				attribute(IFLA_XDP_FD, xdp_extended_bpf_program_file_descriptor.as_raw_fd())
			)
		}
		
		#[inline(always)]
		fn request_1(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>, network_interface_index: NetworkInterfaceIndex, xdp_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, flags: u32) -> Result<(), Errno>
		{
			ExpressDataPathMessageBody::make_request_and_get_acknowledgment_or_error
			(
				netlink_socket_file_descriptor,
				network_interface_index,
				attribute(IFLA_XDP_FD, xdp_extended_bpf_program_file_descriptor.as_raw_fd())
				.followed_by_attribute(IFLA_XDP_FLAGS, flags)
			)
		}
		
		#[inline(always)]
		fn request_2(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>, network_interface_index: NetworkInterfaceIndex, xdp_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor, flags: u32, replace_xdp_extended_bpf_program_file_descriptor: &ExtendedBpfProgramFileDescriptor) -> Result<(), Errno>
		{
			ExpressDataPathMessageBody::make_request_and_get_acknowledgment_or_error
			(
				netlink_socket_file_descriptor,
				network_interface_index,
				attribute(IFLA_XDP_FD, xdp_extended_bpf_program_file_descriptor.as_raw_fd())
				.followed_by_attribute(IFLA_XDP_FLAGS, flags)
				.followed_by_attribute(IFLA_XDP_EXPECTED_FD, replace_xdp_extended_bpf_program_file_descriptor.as_raw_fd())
			)
		}
		
		use self::AttachMode::*;
		use self::UpdateMode::*;
		match (attach_mode, update_mode)
		{
			(GenericOrNative, CreateOrUpdate) => request_0(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor),
			(GenericOrNative, Create) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_UPDATE_IF_NOEXIST),
			(GenericOrNative, Update(replace_xdp_extended_bpf_program_file_descriptor)) => request_2(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_REPLACE, replace_xdp_extended_bpf_program_file_descriptor),
			
			(Generic, CreateOrUpdate) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_SKB_MODE),
			(Generic, Create) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_SKB_MODE | XDP_FLAGS_UPDATE_IF_NOEXIST),
			(Generic, Update(replace_xdp_extended_bpf_program_file_descriptor)) => request_2(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_SKB_MODE | XDP_FLAGS_REPLACE, replace_xdp_extended_bpf_program_file_descriptor),
			
			(Native, CreateOrUpdate) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_DRV_MODE),
			(Native, Create) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_DRV_MODE | XDP_FLAGS_UPDATE_IF_NOEXIST),
			(Native, Update(replace_xdp_extended_bpf_program_file_descriptor)) => request_2(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_DRV_MODE | XDP_FLAGS_REPLACE, replace_xdp_extended_bpf_program_file_descriptor),
			
			(Offloaded, CreateOrUpdate) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_HW_MODE),
			(Offloaded, Create) => request_1(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_HW_MODE | XDP_FLAGS_UPDATE_IF_NOEXIST),
			(Offloaded, Update(replace_xdp_extended_bpf_program_file_descriptor)) => request_2(netlink_socket_file_descriptor, network_interface_index, xdp_extended_bpf_program_file_descriptor, XDP_FLAGS_HW_MODE | XDP_FLAGS_REPLACE, replace_xdp_extended_bpf_program_file_descriptor),
		}
	}
}
