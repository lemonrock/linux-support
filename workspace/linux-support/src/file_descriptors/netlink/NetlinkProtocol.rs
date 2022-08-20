// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Netlink protocol.
pub trait NetlinkProtocol: Debug + Sized
{
	/// Message type.
	type MessageType: Debug;
	
	#[doc(hidden)]
	const Protocol: c_int;
	
	#[doc(hidden)]
	fn message_type(message_type: NetlinkMessageType) -> Self::MessageType;
	
	/// New acknowledge message.
	///
	/// ?Can this can be created by userspace?
	fn new_acknowledge_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkAcknowledgeMessageFlags, body: Body) -> NetlinkRequestMessage<Body>;
	
	/// New delete request message.
	fn new_delete_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkDeleteRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>;
	
	/// New get request message.
	fn new_get_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkGetRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>;
	
	/// New set request message.
	fn new_set_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkSetRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>;
	
	/// New new request message.
	fn new_new_request_message<Body: NetlinkRequestMessageBody>(message_type: Self::MessageType, flags: NetlinkNewRequestMessageFlags, body: Body) -> NetlinkRequestMessage<Body>;
	
	/// Make a a request and get and an acknowledgment or error.
	#[inline(always)]
	fn make_request_and_get_acknowledgment_or_error<Body: NetlinkRequestMessageBody>(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<Self>, mut request: NetlinkRequestMessage<Body>) -> Result<(), SystemCallErrorNumber>
	{
		let sequence_number = netlink_socket_file_descriptor.send_request(&mut request).expect("Send a request");
		
		let message_identification = MultipartMessagePartIdentification::from_linux_kernel(sequence_number);
		
		let mut reply_receiver = AcknowledgmentOnlyReplyReceiver::new(message_identification);
		netlink_socket_file_descriptor.receive_replies(&mut reply_receiver);
		reply_receiver.acknowledgment()
	}
	
}
