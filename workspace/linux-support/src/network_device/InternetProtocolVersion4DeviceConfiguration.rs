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
	///
	/// Range is `0` to `2`.
	pub reverse_path_filter: InternetProtocolVersion4ReversePathFilter,
	
	pub accept_source_route: bool,
	
	pub boot_protocol_relay: bool,
	
	pub log_martians: bool,
	
	pub tag: u32,
	
	pub address_resolution_protocol_filter: bool,
	
	pub address_resolution_protocol_announce: InternetProtocolVersion4AddressResolutionProtocolAnnounce,
	
	pub address_resolution_protocol_ignore: InternetProtocolVersion4AddressResolutionProtocolIgnore,
	
	pub address_resolution_protocol_accept: bool,
	
	pub address_resolution_protocol_notify: bool,
	
	pub drop_gratuitous_address_resolution_protocol: bool,
	
	pub proxy_address_resolution_protocol_pvlan: bool,
	
	/// * `0` is the default and means that the device is the only one on its interface.
	/// * `-1` means the device is not known.
	pub medium_identifier: InternetProtocolVersion4MediumIdentifier,
	
	pub disable_xfrm: bool,
	
	pub disable_policy: bool,
	
	pub promote_secondaries: bool,
	
	pub accept_local: bool,
	
	pub source_valid_mark: u32,
	
	pub route_localnet: bool,
	
	pub force_internet_group_management_protocol_version: InternetProtocolVersion4ForceInternetGroupManagementProtocolVersion,
	
	pub internet_group_management_protocol_version_2_unsolicited_report_interval: Milliseconds,
	
	pub internet_group_management_protocol_version_3_unsolicited_report_interval: Milliseconds,
	
	/// Called `ignore_routes_with_link_down` in `/proc/sys`.
	pub ignore_routes_with_link_down: bool,
	
	/// Called `drop_unicast_in_l2_multicast` in `/proc/sys`.
	pub drop_unicast_in_layer2_multicast: bool,
	
	/// Called `bc_forwarding` in `/proc/sys`.
	pub broadcast_forwarding: bool,
}
