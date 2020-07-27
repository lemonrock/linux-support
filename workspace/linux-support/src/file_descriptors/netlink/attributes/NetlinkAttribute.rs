// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct NetlinkAttribute<V: Sized>
{
	header: nlattr,
	payload: NetlinkAttributePayload<V>,
}

impl<V> NetlinkAttributeOrFollowedByNetlinkAttribute for NetlinkAttribute<V>
{
}

impl<V: NetlinkAttributeOrFollowedByNetlinkAttribute> NetlinkAttribute<V>
{
	#[inline(always)]
	fn nested(netlink_attribute_type: impl NetlinkAttributeType, payload: V) -> Self
	{
		Self::new(netlink_attribute_type.to_u16() | nlattr::NLA_F_NESTED, payload)
	}
}

impl<V: Sized> NetlinkAttribute<V>
{
	#[inline(always)]
	pub(crate) fn followed_by<Following: NetlinkAttributeOrFollowedByNetlinkAttribute>(self, following: Following) -> NetlinkAttributeFollowedByNetlinkAttribute<V, Following>
	{
		NetlinkAttributeFollowedByNetlinkAttribute
		{
			current: self,
			following,
		}
	}
	
	#[inline(always)]
	pub(crate) fn followed_by_attribute<Following: Sized>(self, netlink_attribute_type: impl NetlinkAttributeType, payload: Following) -> NetlinkAttributeFollowedByNetlinkAttribute<V, NetlinkAttribute<Following>>
	{
		self.followed_by(attribute(netlink_attribute_type, payload))
	}
	
	#[inline(always)]
	fn leaf(netlink_attribute_type: impl NetlinkAttributeType, payload: V) -> Self
	{
		Self::new(netlink_attribute_type.to_u16(), payload)
	}
	
	#[inline(always)]
	fn leaf_network_byte_order(netlink_attribute_type: impl NetlinkAttributeType, payload: V) -> Self
	{
		Self::new(netlink_attribute_type.to_u16() | nlattr::NLA_F_NET_BYTEORDER, payload)
	}
	
	#[inline(always)]
	fn new(netlink_attribute_type: u16, payload: V) -> Self
	{
		Self
		{
			header: nlattr
			{
				nla_type: netlink_attribute_type,
				nla_len: nlattr::compute_nla_len::<V>(),
			},
			payload: NetlinkAttributePayload(payload),
		}
	}
}
