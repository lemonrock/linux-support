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
	fn new_new_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkNewRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>
	{
		NetlinkRequestMessage::new_new_request_message(NetlinkMessageType { route: message_type }, flags, body)
	}
}

impl RouteNetlinkProtocol
{
	/// Get Internet Protocol version 4 addresses.
	///
	/// This is ***SLOW***.
	pub fn get_internet_protocol_version_4_addresses(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>) -> Result<Vec<GetAddressMessageData<in_addr>>, String>
	{
		static RouteMessageProcessor: GetAddressRouteMessageProcessor<in_addr> = GetAddressRouteMessageProcessor(PhantomData);
		Self::get_internet_protocol_addresses(netlink_socket_file_descriptor, &RouteMessageProcessor)
	}
	
	/// Get Internet Protocol version 6 addresses.
	///
	/// This is ***SLOW***.
	pub fn get_internet_protocol_version_6_addresses(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>) -> Result<Vec<GetAddressMessageData<in6_addr>>, String>
	{
		static RouteMessageProcessor: GetAddressRouteMessageProcessor<in6_addr> = GetAddressRouteMessageProcessor(PhantomData);
		Self::get_internet_protocol_addresses(netlink_socket_file_descriptor, &RouteMessageProcessor)
	}
	
	#[inline(always)]
	fn get_internet_protocol_addresses<IPA: InternetProtocolAddress>(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>, route_message_processor: &'static GetAddressRouteMessageProcessor<IPA>) -> Result<Vec<GetAddressMessageData<IPA>>, String>
	{
		loop
		{
			let mut request = RouteNetlinkProtocol::new_route_get_internet_protocol_addresses_message::<IPA>();
			let sequence_number = netlink_socket_file_descriptor.send_request(&mut request).expect("Send a request");
			
			let message_identification = MultipartMessagePartIdentification::from_linux_kernel(sequence_number);
			
			match Self::try_receiving_until_get_something(netlink_socket_file_descriptor, route_message_processor, &message_identification)
			{
				Ok(None) => continue,
				
				Ok(Some(processed_messages)) => return Ok(processed_messages),
				
				Err(Left(messaging_parsing_errors)) => return Err(format!("Message parsing errors {:?}", messaging_parsing_errors)),
				
				Err(Right(end_of_set_of_messages_error)) => return Err(format!("End of set of messages errors {:?}", end_of_set_of_messages_error)),
			}
		}
	}
	
	#[inline(always)]
	fn new_route_get_internet_protocol_addresses_message<IPA: InternetProtocolAddress>() -> NetlinkRequestMessage<ifaddrmsg>
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
		Self::new_get_request_message(RouteNetlinkMessageType::GETADDR, NetlinkGetRequestMessageFlags::Dump, body)
	}
	
	
	// TODO: Not sure we need to try receiving more than once but nothing about netlink seems obvious.
	fn try_receiving_until_get_something<IPA: InternetProtocolAddress>(netlink_socket_file_descriptor: &NetlinkSocketFileDescriptor<Self>, route_message_processor: &'static GetAddressRouteMessageProcessor<IPA>, message_identification: &MultipartMessagePartIdentification) -> Result<Option<Vec<GetAddressMessageData<IPA>>>, Either<Vec<String>, io::Error>>
	{
		let mut reply_receiver = RouteReplyReceiver::new(route_message_processor);
		 loop
		{
			netlink_socket_file_descriptor.receive_replies(&mut reply_receiver);
			
			reply_receiver.panic_if_has_could_not_start_messages_errors();
			
			match reply_receiver.messages(message_identification)
			{
				Err(reply_receiver_again) =>
				{
					reply_receiver = reply_receiver_again;
					continue
				}
				
				Ok(something) => return something,
			}
		}
	}
}
