// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ethtool_flow_union
{
	pub(crate) tcp_ip4_spec: ethtool_tcpip4_spec,
	pub(crate) udp_ip4_spec: ethtool_tcpip4_spec,
	pub(crate) sctp_ip4_spec: ethtool_tcpip4_spec,
	pub(crate) ah_ip4_spec: ethtool_ah_espip4_spec,
	pub(crate) esp_ip4_spec: ethtool_ah_espip4_spec,
	pub(crate) usr_ip4_spec: ethtool_usrip4_spec,
	pub(crate) tcp_ip6_spec: ethtool_tcpip6_spec,
	pub(crate) udp_ip6_spec: ethtool_tcpip6_spec,
	pub(crate) sctp_ip6_spec: ethtool_tcpip6_spec,
	pub(crate) ah_ip6_spec: ethtool_ah_espip6_spec,
	pub(crate) esp_ip6_spec: ethtool_ah_espip6_spec,
	pub(crate) usr_ip6_spec: ethtool_usrip6_spec,
	pub(crate) ether_spec: ethhdr,
	pub(crate) hdata: [u8; 52],
}

impl FlowSpecification for ethtool_flow_union
{
}
