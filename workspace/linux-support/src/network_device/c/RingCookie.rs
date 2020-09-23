// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How rings are laid out when accessing virtual functions or offloaded queues is device specific.
///
/// To allow users to do flow steering and specify these queues the ring cookie is partitioned into a 32bit queue index with an 8 bit virtual function id.
/// This also leaves the 3 bytes for further specifiers; it is possible future devices may support more than 256 virtual functions (eg if using PCIe with ARI).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RingCookie(pub(crate) u64);

impl RingCookie
{
	pub(crate) const fn ethtool_get_flow_spec_ring(self) -> u64
	{
		let ring_cookie = self.0;
		ETHTOOL_RX_FLOW_SPEC_RING & ring_cookie
	}
	
	pub(crate) const fn ethtool_get_flow_spec_ring_vf(self) -> u64
	{
		let ring_cookie = self.0;
		(ETHTOOL_RX_FLOW_SPEC_RING_VF & ring_cookie) >> ETHTOOL_RX_FLOW_SPEC_RING_VF_OFF
	}
}
