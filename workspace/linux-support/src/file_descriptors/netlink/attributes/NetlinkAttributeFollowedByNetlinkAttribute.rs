// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct NetlinkAttributeFollowedByNetlinkAttribute<CurrentPayload: Sized, Following: NetlinkAttributeOrFollowedByNetlinkAttribute>
{
	current: NetlinkAttribute<CurrentPayload>,
	following: Following,
}

impl<CurrentPayload: Sized, Following: NetlinkAttributeOrFollowedByNetlinkAttribute> NetlinkAttributeOrFollowedByNetlinkAttribute for NetlinkAttributeFollowedByNetlinkAttribute<CurrentPayload, Following>
{
}

impl<CurrentPayload: Sized, FollowingPayload: Sized> NetlinkAttributeFollowedByNetlinkAttribute<CurrentPayload, NetlinkAttribute<FollowingPayload>>
{
	#[inline(always)]
	pub(crate) fn followed_by<V: NetlinkAttributeOrFollowedByNetlinkAttribute>(self, following: V) -> NetlinkAttributeFollowedByNetlinkAttribute<CurrentPayload, NetlinkAttributeFollowedByNetlinkAttribute<FollowingPayload, V>>
	{
		NetlinkAttributeFollowedByNetlinkAttribute
		{
			current: self.current,
			following: NetlinkAttributeFollowedByNetlinkAttribute
			{
				current: self.following,
				following,
			}
		}
	}
	
	#[inline(always)]
	pub(crate) fn followed_by_attribute<Following: Sized>(self, netlink_attribute_type: impl NetlinkAttributeType, payload: Following) -> NetlinkAttributeFollowedByNetlinkAttribute<CurrentPayload, NetlinkAttributeFollowedByNetlinkAttribute<FollowingPayload, NetlinkAttribute<Following>>>
	{
		self.followed_by(attribute(netlink_attribute_type, payload))
	}
}
