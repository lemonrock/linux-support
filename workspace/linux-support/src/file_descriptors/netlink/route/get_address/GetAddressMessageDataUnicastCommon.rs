// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol unicast address message data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetAddressMessageDataUnicastCommon<IPA: InternetProtocolAddress>
{
	/// Common fields.
	#[serde(flatten)] pub common: GetAddressMessageDataCommon,
	
	/// Source address, and, if this is a point-to-point link, a peer destination address.
	///
	/// In the latter case `common.mask_length_in_bits` should be `Some(IPA::InclusiveMaximumPrefixLength)` but the code in Linux is complex to follow and may result in invalid intermediate states.
	pub source_address_and_point_to_point_peer_destination_address: Option<(IPA, Option<IPA>)>,
	
	/// Route priority.
	pub route_priority: Option<NonZeroU32>,
	
	/// Superset of `self.interface_flags`.
	pub extended_interface_flags: ExtendedInterfaceFlags,
}
