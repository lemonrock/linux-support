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
	/// Get internet protocol addresses.
	///
	/// This is ***SLOW***.
	pub fn get_internet_protocol_addresses<IPA: InternetProtocolAddress>(socket: &NetlinkSocketFileDescriptor<Self>) -> Option<Result<Option<&Vec<GetAddressMessageData<IPA>>>, Either<&Vec<String>, &io::Error>>>
	{
		let mut request = RouteNetlinkProtocol::new_route_get_internet_protocol_addresses_message::<IPA>();
		let sequence_number = socket.send_request(&mut request).expect("Send a request");
		
		static RouteMessageProcessor: GetAddressRouteMessageProcessor<IPA> = GetAddressRouteMessageProcessor::<IPA>(PhantomData);
		
		let mut reply_receiver = RouteReplyReceiver::new(&RouteMessageProcessor);
		socket.receive_replies(&mut reply_receiver);
		reply_receiver.messages(&MultipartMessagePartIdentification::from_linux_kernel(sequence_number))
	}
	
	#[inline(always)]
	fn new_route_get_internet_protocol_addresses_message<IPA: InternetProtocolAddress>() -> NetlinkRequestMessage<ifaddrmsg>
	{
		Self::new_route_get_addresses_message(IPA::AddressFamily)
	}
	
	#[inline(always)]
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
			ifa_index: 0,
			
			// Must all be zero (seems mad that the same struct is used for both requests and replies).
			ifa_prefixlen: unsafe { zeroed() },
			ifa_flags: unsafe { zeroed() },
			ifa_scope: unsafe { zeroed() },
		};
		Self::new_get_request_message(RouteNetlinkMessageType::GETADDR, NetlinkGetRequestMessageFlags::Dump, body)
	}
}
