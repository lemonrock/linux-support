// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct ExpressDataPathMessageBody<V: NetlinkAttributeOrFollowedByNetlinkAttribute>
{
	ifinfo: ifinfomsg,
	nested_attributes: NetlinkAttribute<V>,
}

impl<V: NetlinkAttributeOrFollowedByNetlinkAttribute> NetlinkRequestMessageBody for ExpressDataPathMessageBody<V>
{
	#[inline(always)]
	fn family(&self) -> c_uchar
	{
		self.ifinfo.ifi_family
	}
}

impl<V: NetlinkAttributeOrFollowedByNetlinkAttribute> ExpressDataPathMessageBody<V>
{
	#[inline(always)]
	pub(super) fn make_request_and_get_acknowledgment_or_error(netlink_socket_file_descriptor: &mut NetlinkSocketFileDescriptor<RouteNetlinkProtocol>, network_interface_index: NetworkInterfaceIndex, payload: V) -> Result<(), Errno>
	{
		use self::IFLA::*;
		
		let message_body = Self
		{
			ifinfo: ifinfomsg::for_xdp(network_interface_index),
			nested_attributes: IFLA_XDP.nests(payload),
		};
		
		let request = RouteNetlinkProtocol::new_set_request_message(RouteNetlinkMessageType::SETLINK, NetlinkSetRequestMessageFlags::empty(), message_body);
		NetlinkProtocol::make_request_and_get_acknowledgment_or_error(netlink_socket_file_descriptor, request)
	}
}
