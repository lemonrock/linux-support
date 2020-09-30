// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Address message data common to different messages.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetAddressMessageDataCommon
{
	/// Validated to be under exclusive maximum possible for `InternetProtocolAddress`.
	///
	/// `1 ..= 32` for IPv4.
	/// `1 ..= 128` for IPv6.
	pub mask_length_in_bits: Option<NonZeroU8>,
	
	/// Zone identifier (interface id) used for link-local addressing.
	/// Known as `sin6_scope_id` in the libc struct `sockaddr_in6`.
	/// See <https://tools.ietf.org/html/rfc6874>.
	/// Examples of the value (after the `%`) are:-
	///
	/// * `fe80::1:2:3:4%9` ie `9`
	/// * `fe80::1:2:3:4%eth0` ie `eth0`.
	///
	/// This is the index of the interface with the associated address(es).
	///
	/// Only relevant to IPv6 if the `IN6_IS_ADDR_LINKLOCAL()` or `IN6_IS_ADDR_MC_LINKLOCAL()` macros are true.
	pub network_interface_index: NetworkInterfaceIndex,
	
	#[allow(missing_docs)]
	pub address_scope: rt_scope,
	
	#[allow(missing_docs)]
	pub target_net_namespace_identifier: Option<NetNamespaceIdentifer>,
	
	#[allow(missing_docs)]
	pub temporary_address_valid_lifetime: InternetProtocolAddressLifetime,
	
	#[allow(missing_docs)]
	pub temporary_address_prefered_lifetime: InternetProtocolAddressLifetime,
	
	#[allow(missing_docs)]
	pub temporary_address_created_timestamp: CacheTimestampInHundrethsOfSeconds,
	
	#[allow(missing_docs)]
	pub temporary_address_updated_timestamp: CacheTimestampInHundrethsOfSeconds,
}
