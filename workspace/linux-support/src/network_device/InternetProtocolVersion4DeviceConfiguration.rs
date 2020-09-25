// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol version 4 device configuration.
///
/// All of these values should be present in `/proc/sys`.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InternetProtocolVersion4DeviceConfiguration
{
	/// Complex value see `devinet_sysctl_forward()`.
	pub forwarding: u32,
	
	/// Read-only.
	///
	/// Called `mc_forwarding` in `/proc/sys`.
	pub multicast_forwarding: bool,
	
	pub proxy_arp: bool,
	
	pub accept_redirects: bool,
	
	pub secure_redirects: bool,
	
	pub send_redirects: bool,
	
	pub shared_media: u32,
	
	/// Called `rp_filter` in `/proc/sys`.
	pub reverse_path_filter: bool,
	
	pub accept_source_route: bool,
	
	pub bootp_relay: bool,
	
	pub log_martians: bool,
	
	pub tag: u32,
	
	pub arp_filter: bool,
	
	pub medium_id: u32,
	
	pub disable_xfrm: bool,
	
	pub disable_policy: bool,
	
	pub force_igmp_version: bool,
	
	pub arp_announce: bool,
	
	pub arp_ignore: bool,
	
	pub promote_secondaries: bool,
	
	pub arp_accept: bool,
	
	pub arp_notify: bool,
	
	pub accept_local: bool,
	
	pub source_valid_mark: u32,
	
	pub proxy_arp_pvlan: bool,
	
	pub route_localnet: bool,
	
	pub igmpv2_unsolicited_report_interval: u32,
	
	pub igmpv3_unsolicited_report_interval: u32,
	
	/// Called `ignore_routes_with_link_down` in `/proc/sys`.
	pub ignore_routes_with_link_down: bool,
	
	/// Called `drop_unicast_in_l2_multicast` in `/proc/sys`.
	pub drop_unicast_in_layer2_multicast: bool,
	
	pub drop_gratuitous_arp: bool,
	
	/// Called `bc_forwarding` in `/proc/sys`.
	pub broadcast_forwarding: bool,
}
