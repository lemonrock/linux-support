// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol version 6 device configuration.
///
/// All of these values should be present in `/proc/sys`.
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InternetProtocolVersion6DeviceConfiguration
{
	/// `disable_ipv6`.
	///
	/// Default is `0`.
	pub disable_ipv6: bool,
	
	/// `forwarding`.
	///
	/// Default is `0`.
	pub forwarding: bool,
	
	/// `hop_limit`.
	///
	/// Default if `IPV6_DEFAULT_HOPLIMIT`.
	pub hop_limit: u32,
	
	/// `maximum_transmission_unit6`.
	///
	/// Default is `IPV6_MIN_MTU`.
	pub maximum_transmission_unit6: u32,
	
	/// `autoconf`.
	///
	/// Default is `1`.
	pub autoconf: bool,
	
	/// `router_solicits`.
	///
	/// Default is `MAX_RTR_SOLICITATIONS`.
	pub router_solicits: u32,
	
	/// `router_solicit_interval`.
	///
	/// Milliseconds.
	///
	/// Default is `RTR_SOLICITATION_INTERVAL`.
	pub router_solicit_interval: Milliseconds,
	
	/// `router_solicit_maximum_interval`.
	///
	/// Milliseconds.
	///
	/// Default is `RTR_SOLICITATION_MAX_INTERVAL`.
	pub router_solicit_maximum_interval: Milliseconds,
	
	/// `router_solicit_delay`.
	///
	/// Milliseconds.
	///
	/// Default is `MAX_RTR_SOLICITATION_DELAY`.
	pub router_solicit_delay: Milliseconds,
	
	/// `use_temporary_address`.
	///
	/// Default is `0`.
	pub use_temporary_address: bool,
	
	/// `temporary_address_valid_lifetime`.
	///
	/// Default is `TEMP_VALID_LIFETIME`.
	/// Infinity is `INFINITY_LIFE_TIME` (`0xFFFF_FFFF`).
	pub temporary_address_valid_lifetime: u32,
	
	/// `temporary_address_prefered_lifetime`.
	///
	/// Default is `TEMP_PREFERRED_LIFETIME`.
	/// Infinity is `INFINITY_LIFE_TIME` (`0xFFFF_FFFF`).
	pub temporary_address_prefered_lifetime: u32,
	
	/// `regen_maximum_retry`.
	///
	/// Default is `REGEN_MAX_RETRY`.
	pub regen_maximum_retry: u32,
	
	/// `maximum_desync_factor`.
	///
	/// Default is `MAX_DESYNC_FACTOR`.
	pub maximum_desync_factor: u32,
	
	/// `maximum_addresses`.
	///
	/// Default is `IPV6_MAX_ADDRESSES`.
	pub maximum_addresses: u32,
	
	/// `accept_router_advertisement`.
	///
	/// Default is `1`.
	pub accept_router_advertisement: bool,
	
	/// `accept_redirects`.
	///
	/// Default is `1`.
	pub accept_redirects: bool,
	
	/// `accept_router_advertisement_default_router`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_default_router: bool,
	
	/// `accept_router_advertisement_from_local`.
	///
	/// Default is `0`.
	pub accept_router_advertisement_from_local: bool,
	
	/// `accept_router_advertisement_minimum_hop_limit`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_minimum_hop_limit: bool,
	
	/// `accept_router_advertisement_maximum_transmission_unit`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_maximum_transmission_unit: bool,
	
	/// `accept_router_advertisement_prefix_information`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_prefix_information: bool,
	
	/// `accept_router_advertisement_route_information_maximum_prefix_length`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTE_INFO`.
	///
	/// Default is `Some(0)`.
	pub accept_router_advertisement_route_information_maximum_prefix_length: Option<u32>,
	
	/// `accept_router_advertisement_route_information_minimum_prefix_length`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTE_INFO`.
	///
	/// Default is `Some(0)`.
	pub accept_router_advertisement_route_information_minimum_prefix_length: Option<u32>,
	
	/// `accept_router_advertisement_router_preference`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTER_PREF`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_router_preference: Option<bool>,
	
	/// `router_probe_interval`.
	///
	/// Milliseconds.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTER_PREF`.
	///
	/// Default is `Some(60 * Hz)`.
	pub router_probe_interval: Option<Milliseconds>,
	
	/// `duplicate_address_detection_transmits`.
	///
	/// Default is `1`.
	pub duplicate_address_detection_transmits: bool,
	
	/// `accept_duplicate_address_detection`.
	///
	/// Default is `1`.
	pub accept_duplicate_address_detection: bool,
	
	/// `enhanced_duplicate_address_detection`.
	///
	/// Default is `1`.
	pub enhanced_duplicate_address_detection: bool,
	
	/// `use_optimistic_duplicate_address_detection`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_OPTIMISTIC_DAD`.
	///
	/// No default (so probably `0`).
	pub use_optimistic_duplicate_address_detection: Option<bool>,
	
	/// `optimistic_duplicate_address_detection`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_OPTIMISTIC_DAD`.
	///
	/// No default (so probably `0`).
	pub optimistic_duplicate_address_detection: Option<bool>,
	
	/// `accept_source_route`.
	///
	/// Default is `0`.
	pub accept_source_route: bool,
	
	/// `mulitcast_forwarding`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_MROUTE`.
	pub mulitcast_forwarding: Option<bool>,
	
	/// `force_force_target_link_layer_address_option`.
	///
	/// No default (so probably `0`).
	pub force_force_target_link_layer_address_option: bool,
	
	/// `proxy_neighbor_discovery_protocol`.
	///
	/// Default is `0`.
	pub proxy_neighbor_discovery_protocol: bool,
	
	/// `icmpv6_neighbor_discovery_notify`.
	///
	/// No default (so probably `0`).
	pub icmpv6_neighbor_discovery_notify: bool,
	
	/// `icmpv6_neighbor_discovery_suppress_fragments`.
	///
	/// Default is `1`.
	pub icmpv6_neighbor_discovery_suppress_fragments: bool,
	
	/// `icmpv6_neighbor_discovery_traffic_class`.
	///
	/// No default (so probably `0`).
	pub icmpv6_neighbor_discovery_traffic_class: u32,
	
	/// `force_multicast_listener_discovery_version`.
	///
	/// Default is `0`.
	pub force_multicast_listener_discovery_version: bool,
	
	/// `multicast_listener_discovery_v1_unsolicited_report_interval`.
	///
	/// Milliseconds.
	///
	/// Default is `10 * HZ`.
	/// `HZ` is defined as `100`.
	pub multicast_listener_discovery_v1_unsolicited_report_interval: Milliseconds,
	
	/// `multicast_listener_discovery_v2_unsolicited_report_interval`.
	///
	/// Milliseconds.
	///
	/// Default is `HZ`.
	/// `HZ` is defined as `100`.
	pub multicast_listener_discovery_v2_unsolicited_report_interval: Milliseconds,
	
	/// `use_output_interface_addresses_only`.
	///
	/// Default is `0`.
	pub use_output_interface_addresses_only: bool,
	
	/// `ignore_routes_with_link_down`.
	///
	/// Default is `0`.
	pub ignore_routes_with_link_down: bool,
	
	/// `drop_unicast_in_layer2_multicast`.
	///
	/// No default (so probably `0`).
	pub drop_unicast_in_layer2_multicast: bool,
	
	/// `drop_unsolicited_neighbor_advertisements`.
	///
	/// No default (so probably `0`).
	pub drop_unsolicited_neighbor_advertisements: bool,
	
	/// `keep_address_on_down`.
	///
	/// Default is `0`.
	pub keep_address_on_down: bool,
	
	/// `seg6_enabled`.
	///
	/// Default is `0`.
	pub seg6_enabled: bool,
	
	/// `seg6_require_hmac`.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_SEG6_HMAC`.
	///
	/// Default is `Some(0)`.
	pub seg6_require_hmac: Option<bool>,
	
	/// `address_generation_mode`.
	///
	/// Default is `IN6_ADDR_GEN_MODE_EUI64`.
	pub address_generation_mode: in6_addr_gen_mode,
	
	/// `disable_policy`.
	///
	/// Default is `0`.
	pub disable_policy: bool,
	
	/// `rpl_seg_enabled`.
	///
	/// Default is `0`.
	pub rpl_seg_enabled: bool,
}
