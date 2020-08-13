// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait NetlinkAttributeType: Debug + Copy + PartialEq + Eq + PartialOrd + Ord + Hash
{
	#[doc(hidden)]
	fn to_u16(self) -> u16;
	
	#[inline(always)]
	fn nests<V: NetlinkAttributeOrFollowedByNetlinkAttribute>(self, payload: V) -> NetlinkAttribute<V>
	{
		NetlinkAttribute::nested(self, payload)
	}
	
	#[inline(always)]
	fn attribute<V: Sized>(self, payload: V) -> NetlinkAttribute<V>
	{
		NetlinkAttribute::leaf(self, payload)
	}
	
	#[inline(always)]
	fn attribute_network_byte_order<V: Sized>(self, payload: V) -> NetlinkAttribute<V>
	{
		NetlinkAttribute::leaf_network_byte_order(self, payload)
	}
}
