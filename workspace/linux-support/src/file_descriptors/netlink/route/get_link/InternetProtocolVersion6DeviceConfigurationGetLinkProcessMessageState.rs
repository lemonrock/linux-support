// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct InternetProtocolVersion6DeviceConfigurationGetLinkProcessMessageState
{
	pub(crate) disable_ipv6: Option<bool>,
	
	pub(crate) forwarding: Option<bool>,
	
	pub(crate) hop_limit: Option<u32>,
	
	pub(crate) maximum_transmission_unit: Option<MaximumTransmissionUnitPayloadSize>,
	
	pub(crate) autoconf: Option<bool>,
	
	pub(crate) router_solicits: Option<u32>,
	
	pub(crate) router_solicit_interval: Option<Milliseconds>,
	
	pub(crate) router_solicit_maximum_interval: Option<Milliseconds>,
	
	pub(crate) router_solicit_delay: Option<Milliseconds>,
	
	pub(crate) use_temporary_address: Option<bool>,
	
	pub(crate) temporary_address_valid_lifetime: Option<InternetProtocolAddressLifetime>,
	
	pub(crate) temporary_address_prefered_lifetime: Option<InternetProtocolAddressLifetime>,
	
	pub(crate) regen_maximum_retry: Option<u32>,
	
	pub(crate) maximum_desync_factor: Option<u32>,
	
	pub(crate) maximum_addresses: Option<u32>,
	
	pub(crate) accept_router_advertisement: Option<bool>,
	
	pub(crate) accept_redirects: Option<bool>,
	
	pub(crate) accept_router_advertisement_default_router: Option<bool>,
	
	pub(crate) accept_router_advertisement_from_local: Option<bool>,
	
	pub(crate) accept_router_advertisement_minimum_hop_limit: Option<bool>,
	
	pub(crate) accept_router_advertisement_maximum_transmission_unit: Option<bool>,
	
	pub(crate) accept_router_advertisement_prefix_information: Option<bool>,
	
	pub(crate) accept_router_advertisement_route_information_maximum_prefix_length: Option<Option<bool>>,
	
	pub(crate) accept_router_advertisement_route_information_minimum_prefix_length: Option<Option<bool>>,
	
	pub(crate) accept_router_advertisement_router_preference: Option<Option<bool>>,
	
	pub(crate) router_probe_interval: Option<Option<Milliseconds>>,
	
	pub(crate) duplicate_address_detection_transmits: Option<bool>,
	
	pub(crate) accept_duplicate_address_detection: Option<bool>,
	
	pub(crate) enhanced_duplicate_address_detection: Option<bool>,
	
	pub(crate) use_optimistic_duplicate_address_detection: Option<Option<bool>>,
	
	pub(crate) optimistic_duplicate_address_detection: Option<Option<bool>>,
	
	pub(crate) accept_source_route: Option<bool>,
	
	pub(crate) mulitcast_forwarding: Option<Option<bool>>,
	
	pub(crate) force_force_target_link_layer_address_option: Option<bool>,
	
	pub(crate) proxy_neighbor_discovery_protocol: Option<bool>,
	
	pub(crate) icmpv6_neighbor_discovery_notify: Option<bool>,
	
	pub(crate) icmpv6_neighbor_discovery_suppress_fragments: Option<bool>,
	
	pub(crate) icmpv6_neighbor_discovery_traffic_class: Option<u32>,
	
	pub(crate) force_multicast_listener_discovery_version: Option<bool>,
	
	pub(crate) multicast_listener_discovery_v1_unsolicited_report_interval: Option<Milliseconds>,
	
	pub(crate) multicast_listener_discovery_v2_unsolicited_report_interval: Option<Milliseconds>,
	
	pub(crate) use_output_interface_addresses_only: Option<bool>,
	
	pub(crate) ignore_routes_with_link_down: Option<bool>,
	
	pub(crate) drop_unicast_in_layer2_multicast: Option<bool>,
	
	pub(crate) drop_unsolicited_neighbor_advertisements: Option<bool>,
	
	pub(crate) keep_address_on_down: Option<bool>,
	
	pub(crate) seg6_enabled: Option<bool>,
	
	pub(crate) seg6_require_hmac: Option<Option<bool>>,
	
	pub(crate) address_generation_mode: Option<in6_addr_gen_mode>,
	
	pub(crate) disable_policy: Option<bool>,
	
	pub(crate) rpl_seg_enabled: Option<bool>,
}

impl InternetProtocolVersion6DeviceConfigurationGetLinkProcessMessageState
{
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<InternetProtocolVersion6DeviceConfiguration, String>
	{
		Ok
		(
			InternetProtocolVersion6DeviceConfiguration
			{
				disable_ipv6: self.disable_ipv6.ok_or(format!("Linux kernel bug - missing disable_ipv6"))?,
				
				forwarding: self.forwarding.ok_or(format!("Linux kernel bug - missing forwarding"))?,
				
				hop_limit: self.hop_limit.ok_or(format!("Linux kernel bug - missing hop_limit"))?,
				
				maximum_transmission_unit: self.maximum_transmission_unit.ok_or(format!("Linux kernel bug - missing maximum_transmission_unit6"))?,
				
				autoconf: self.autoconf.ok_or(format!("Linux kernel bug - missing autoconf"))?,
				
				router_solicits: self.router_solicits.ok_or(format!("Linux kernel bug - missing router_solicits"))?,
				
				router_solicit_interval: self.router_solicit_interval.ok_or(format!("Linux kernel bug - missing router_solicit_interval"))?,
				
				router_solicit_maximum_interval: self.router_solicit_maximum_interval.ok_or(format!("Linux kernel bug - missing router_solicit_maximum_interval"))?,
				
				router_solicit_delay: self.router_solicit_delay.ok_or(format!("Linux kernel bug - missing router_solicit_delay"))?,
				
				use_temporary_address: self.use_temporary_address.ok_or(format!("Linux kernel bug - missing use_temporary_address"))?,
				
				temporary_address_valid_lifetime: self.temporary_address_valid_lifetime.ok_or(format!("Linux kernel bug - missing temporary_address_valid_lifetime"))?,
				
				temporary_address_prefered_lifetime: self.temporary_address_prefered_lifetime.ok_or(format!("Linux kernel bug - missing temporary_address_prefered_lifetime"))?,
				
				regen_maximum_retry: self.regen_maximum_retry.ok_or(format!("Linux kernel bug - missing regen_maximum_retry"))?,
				
				maximum_desync_factor: self.maximum_desync_factor.ok_or(format!("Linux kernel bug - missing maximum_desync_factor"))?,
				
				maximum_addresses: self.maximum_addresses.ok_or(format!("Linux kernel bug - missing maximum_addresses"))?,
				
				accept_router_advertisement: self.accept_router_advertisement.ok_or(format!("Linux kernel bug - missing accept_router_advertisement"))?,
				
				accept_redirects: self.accept_redirects.ok_or(format!("Linux kernel bug - missing accept_redirects"))?,
				
				accept_router_advertisement_default_router: self.accept_router_advertisement_default_router.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_default_router"))?,
				
				accept_router_advertisement_from_local: self.accept_router_advertisement_from_local.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_from_local"))?,
				
				accept_router_advertisement_minimum_hop_limit: self.accept_router_advertisement_minimum_hop_limit.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_minimum_hop_limit"))?,
				
				accept_router_advertisement_maximum_transmission_unit: self.accept_router_advertisement_maximum_transmission_unit.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_maximum_transmission_unit"))?,
				
				accept_router_advertisement_prefix_information: self.accept_router_advertisement_prefix_information.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_prefix_information"))?,
				
				accept_router_advertisement_route_information_maximum_prefix_length: self.accept_router_advertisement_route_information_maximum_prefix_length.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_route_information_maximum_prefix_length"))?,
				
				accept_router_advertisement_route_information_minimum_prefix_length: self.accept_router_advertisement_route_information_minimum_prefix_length.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_route_information_minimum_prefix_length"))?,
				
				accept_router_advertisement_router_preference: self.accept_router_advertisement_router_preference.ok_or(format!("Linux kernel bug - missing accept_router_advertisement_router_preference"))?,
				
				router_probe_interval: self.router_probe_interval.ok_or(format!("Linux kernel bug - missing router_probe_interval"))?,
				
				duplicate_address_detection_transmits: self.duplicate_address_detection_transmits.ok_or(format!("Linux kernel bug - missing duplicate_address_detection_transmits"))?,
				
				accept_duplicate_address_detection: self.accept_duplicate_address_detection.ok_or(format!("Linux kernel bug - missing accept_duplicate_address_detection"))?,
				
				enhanced_duplicate_address_detection: self.enhanced_duplicate_address_detection.ok_or(format!("Linux kernel bug - missing enhanced_duplicate_address_detection"))?,
				
				use_optimistic_duplicate_address_detection: self.use_optimistic_duplicate_address_detection.ok_or(format!("Linux kernel bug - missing use_optimistic_duplicate_address_detection"))?,
				
				optimistic_duplicate_address_detection: self.optimistic_duplicate_address_detection.ok_or(format!("Linux kernel bug - missing optimistic_duplicate_address_detection"))?,
				
				accept_source_route: self.accept_source_route.ok_or(format!("Linux kernel bug - missing accept_source_route"))?,
				
				mulitcast_forwarding: self.mulitcast_forwarding.ok_or(format!("Linux kernel bug - missing mulitcast_forwarding"))?,
				
				force_force_target_link_layer_address_option: self.force_force_target_link_layer_address_option.ok_or(format!("Linux kernel bug - missing force_force_target_link_layer_address_option"))?,
				
				proxy_neighbor_discovery_protocol: self.proxy_neighbor_discovery_protocol.ok_or(format!("Linux kernel bug - missing proxy_neighbor_discovery_protocol"))?,
				
				icmpv6_neighbor_discovery_notify: self.icmpv6_neighbor_discovery_notify.ok_or(format!("Linux kernel bug - missing icmpv6_neighbor_discovery_notify"))?,
				
				icmpv6_neighbor_discovery_suppress_fragments: self.icmpv6_neighbor_discovery_suppress_fragments.ok_or(format!("Linux kernel bug - missing icmpv6_neighbor_discovery_suppress_fragments"))?,
				
				icmpv6_neighbor_discovery_traffic_class: self.icmpv6_neighbor_discovery_traffic_class.ok_or(format!("Linux kernel bug - missing icmpv6_neighbor_discovery_traffic_class"))?,
				
				force_multicast_listener_discovery_version: self.force_multicast_listener_discovery_version.ok_or(format!("Linux kernel bug - missing force_multicast_listener_discovery_version"))?,
				
				multicast_listener_discovery_v1_unsolicited_report_interval: self.multicast_listener_discovery_v1_unsolicited_report_interval.ok_or(format!("Linux kernel bug - missing multicast_listener_discovery_v1_unsolicited_report_interval"))?,
				
				multicast_listener_discovery_v2_unsolicited_report_interval: self.multicast_listener_discovery_v2_unsolicited_report_interval.ok_or(format!("Linux kernel bug - missing multicast_listener_discovery_v2_unsolicited_report_interval"))?,
				
				use_output_interface_addresses_only: self.use_output_interface_addresses_only.ok_or(format!("Linux kernel bug - missing use_output_interface_addresses_only"))?,
				
				ignore_routes_with_link_down: self.ignore_routes_with_link_down.ok_or(format!("Linux kernel bug - missing ignore_routes_with_link_down"))?,
				
				drop_unicast_in_layer2_multicast: self.drop_unicast_in_layer2_multicast.ok_or(format!("Linux kernel bug - missing drop_unicast_in_layer2_multicast"))?,
				
				drop_unsolicited_neighbor_advertisements: self.drop_unsolicited_neighbor_advertisements.ok_or(format!("Linux kernel bug - missing drop_unsolicited_neighbor_advertisements"))?,
				
				keep_address_on_down: self.keep_address_on_down.ok_or(format!("Linux kernel bug - missing keep_address_on_down"))?,
				
				seg6_enabled: self.seg6_enabled.ok_or(format!("Linux kernel bug - missing seg6_enabled"))?,
				
				seg6_require_hmac: self.seg6_require_hmac.ok_or(format!("Linux kernel bug - missing seg6_require_hmac"))?,
				
				address_generation_mode: self.address_generation_mode.ok_or(format!("Linux kernel bug - missing address_generation_mode"))?,
				
				disable_policy: self.disable_policy.ok_or(format!("Linux kernel bug - missing disable_policy"))?,
				
				rpl_seg_enabled: self.rpl_seg_enabled.ok_or(format!("Linux kernel bug - missing rpl_seg_enabled"))?,
			}
		)
	}
}
