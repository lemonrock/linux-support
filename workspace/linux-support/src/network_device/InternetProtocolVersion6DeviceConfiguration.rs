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
	/// Default is `0`.
	pub disable_ipv6: bool,
	
	/// Default is `0`.
	pub forwarding: bool,
	
	/// Default is `IPV6_DEFAULT_HOPLIMIT` (64).
	pub hop_limit: u32,
	
	/// Default is `IPV6_MIN_MTU` but usually same as ethernet card `1500` or `1280`.
	pub maximum_transmission_unit: MaximumTransmissionUnitPayloadSize,
	
	/// Default is `1`.
	pub autoconf: bool,
	
	/// Default is `MAX_RTR_SOLICITATIONS`.
	pub router_solicits: u32,
	
	/// Milliseconds.
	///
	/// Default is `RTR_SOLICITATION_INTERVAL`.
	pub router_solicit_interval: Milliseconds,
	
	/// Milliseconds.
	///
	/// Default is `RTR_SOLICITATION_MAX_INTERVAL`.
	pub router_solicit_maximum_interval: Milliseconds,
	
	/// Milliseconds.
	///
	/// Default is `MAX_RTR_SOLICITATION_DELAY`.
	pub router_solicit_delay: Milliseconds,
	
	/// See `use_tempaddr` in <https://www.kernel.org/doc/html/latest/networking/ip-sysctl.html#proc-sys-net-ipv6-variables>.
	///
	/// Default is `0`.
	pub use_temporary_address: InternetProtocolVersion6PrivacyExtensions,
	
	/// Default is `TEMP_VALID_LIFETIME` (`7 × 86,400`).
	/// Infinity is `INFINITY_LIFE_TIME` (`0xFFFF_FFFF`).
	pub temporary_address_valid_lifetime: InternetProtocolAddressLifetime,
	
	/// Default is `TEMP_PREFERRED_LIFETIME` (`86,400`).
	/// Infinity is `INFINITY_LIFE_TIME` (`0xFFFF_FFFF`).
	pub temporary_address_preferred_lifetime: InternetProtocolAddressLifetime,
	
	/// Default is `REGEN_MAX_RETRY`.
	pub regen_maximum_retry: u32,
	
	/// Default is `MAX_DESYNC_FACTOR` (600).
	pub maximum_desync_factor: u32,
	
	/// Default is `IPV6_MAX_ADDRESSES` (16).
	pub maximum_addresses: u32,
	
	/// Default is `1`.
	pub accept_router_advertisement: InternetProtocolVersion6AcceptRouterAdvertisement,
	
	/// Default is `1`.
	pub accept_redirects: bool,
	
	/// Default is `1`.
	pub accept_router_advertisement_default_router: bool,
	
	/// Default is `0`.
	pub accept_router_advertisement_from_local: bool,
	
	/// Default is `1`.
	pub accept_router_advertisement_minimum_hop_limit: bool,
	
	/// Default is `1`.
	pub accept_router_advertisement_maximum_transmission_unit: bool,
	
	/// Default is `1`.
	pub accept_router_advertisement_prefix_information: bool,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTE_INFO`.
	///
	/// Default is `Some(0)`.
	pub accept_router_advertisement_route_information_maximum_prefix_length: Option<bool>,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTE_INFO`.
	///
	/// Default is `Some(0)`.
	pub accept_router_advertisement_route_information_minimum_prefix_length: Option<bool>,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTER_PREF`.
	///
	/// Default is `1`.
	pub accept_router_advertisement_router_preference: Option<bool>,
	
	/// Milliseconds.
	///
	/// Only present if Linux compiled with `CONFIG_IPV6_ROUTER_PREF`.
	///
	/// Default is `Some(60 * Hz)`.
	pub router_probe_interval: Option<Milliseconds>,
	
	/// Default is `1`.
	pub duplicate_address_detection_transmits: u32,
	
	/// Range is `0` to `2`.
	///
	/// Default is `1`.
	pub accept_duplicate_address_detection: InternetProtocolVersion6AcceptDuplicateAddressDetection,
	
	/// Default is `1`.
	pub enhanced_duplicate_address_detection: bool,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_OPTIMISTIC_DAD`.
	///
	/// No default (so probably `0`).
	pub use_optimistic_duplicate_address_detection: Option<bool>,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_OPTIMISTIC_DAD`.
	///
	/// No default (so probably `0`).
	pub optimistic_duplicate_address_detection: Option<bool>,
	
	/// Default is `0`.
	pub accept_source_route: bool,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_MROUTE`.
	pub mulitcast_forwarding: Option<bool>,
	
	/// No default (so probably `0`).
	pub force_force_target_link_layer_address_option: bool,
	
	/// Default is `0`.
	pub proxy_neighbor_discovery_protocol: bool,
	
	/// No default (so probably `0`).
	pub icmpv6_neighbor_discovery_notify: bool,
	
	/// Default is `1`.
	pub icmpv6_neighbor_discovery_discard_fragmented_packets: bool,
	
	/// Default is `0`.
	pub icmpv6_neighbor_discovery_traffic_class: u8,
	
	/// Range is `0` to `2`.
	///
	/// Default is `0`.
	pub force_multicast_listener_discovery_version: InternetProtocolVersion6ForceMulticastListenerDiscoverVersion,
	
	/// Milliseconds.
	///
	/// Default is 10,000 milliseconds.
	pub multicast_listener_discovery_v1_unsolicited_report_interval: Milliseconds,
	
	/// Milliseconds.
	///
	/// Default is 1,000 milliseconds.
	pub multicast_listener_discovery_v2_unsolicited_report_interval: Milliseconds,
	
	/// Default is `0`.
	pub use_output_interface_addresses_only: bool,
	
	/// Default is `0`.
	pub ignore_routes_with_link_down: bool,
	
	/// No default (so probably `0`).
	pub drop_unicast_in_layer2_multicast: bool,
	
	/// No default (so probably `0`).
	pub drop_unsolicited_neighbor_advertisements: bool,
	
	/// See `keep_addr_on_down` in <https://www.kernel.org/doc/html/latest/networking/ip-sysctl.html#proc-sys-net-ipv6-variables>.
	///
	/// Default is `0`.
	pub keep_address_on_down: InternetProtocolVersion6KeepAddressOnDown,
	
	/// Default is `0`.
	pub seg6_enabled: bool,
	
	/// Only present if Linux compiled with `CONFIG_IPV6_SEG6_HMAC`.
	///
	/// Default is `Some(0)`.
	pub seg6_hmac_policy: Option<HmacPolicyForSrEnabledPackets>,
	
	/// Default is `IN6_ADDR_GEN_MODE_EUI64`.
	pub address_generation_mode: in6_addr_gen_mode,
	
	/// Default is `0`.
	pub disable_policy: bool,
	
	/// Default is `0`.
	pub rpl_seg_enabled: bool,
}
