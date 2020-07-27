// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A netlink request message.
#[repr(C)]
pub struct NetlinkRequestMessage<Body: NetlinkRequestMessageBody>
{
	pub(super) header: nlmsghdr,
	body: Body,
}

impl<Body: NetlinkRequestMessageBody> NetlinkRequestMessage<Body>
{
	#[inline(always)]
	pub(super) fn length(&self) -> usize
	{
		self.header.length()
	}
	
	/// ?Can this can be created by userspace?
	#[inline(always)]
	pub(super) fn new_acknowledge_message(message_type: NetlinkMessageType, flags: NetlinkAcknowledgeMessageFlags, body: Body) -> Self
	{
		NetlinkRequestMessage::new_request_message(message_type, NetlinkSpecificMessageFlags { acknowledge: flags }, body)
	}
	
	#[inline(always)]
	pub(super) fn new_delete_request_message(message_type: NetlinkMessageType, flags: NetlinkDeleteRequestMessageFlags, body: Body) -> Self
	{
		NetlinkRequestMessage::new_request_message(message_type, NetlinkSpecificMessageFlags { delete_request: flags }, body)
	}
	
	#[inline(always)]
	pub(super) fn new_get_request_message(message_type: NetlinkMessageType, flags: NetlinkGetRequestMessageFlags, body: Body) -> Self
	{
		NetlinkRequestMessage::new_request_message(message_type, NetlinkSpecificMessageFlags { get_request: flags }, body)
	}
	
	#[inline(always)]
	pub(super) fn new_set_request_message(message_type: NetlinkMessageType, flags: NetlinkSetRequestMessageFlags, body: Body) -> Self
	{
		NetlinkRequestMessage::new_request_message(message_type, NetlinkSpecificMessageFlags { set_request: flags }, body)
	}
	
	#[inline(always)]
	pub(super) fn new_new_request_message(message_type: NetlinkMessageType, flags: NetlinkNewRequestMessageFlags, body: Body) -> Self
	{
		NetlinkRequestMessage::new_request_message(message_type, NetlinkSpecificMessageFlags { new_request: flags }, body)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn new_request_message(message_type: NetlinkMessageType, netlink_specific_message_flags: NetlinkSpecificMessageFlags, body: Body) -> Self
	{
		Self
		{
			header: nlmsghdr
			{
				nlmsg_len: nlmsghdr::NLMSG_LENGTH(size_of::<Body>()) as u32,
				nlmsg_type: message_type,
				nlmsg_flags: NetlinkMessageFlags::new(NetlinkCommonMessageFlags::Request, netlink_specific_message_flags),
				nlmsg_seq: unsafe { uninitialized() },
				nlmsg_pid: PortIdentifier::current_process(),
			},
			body,
		}
	}
}
