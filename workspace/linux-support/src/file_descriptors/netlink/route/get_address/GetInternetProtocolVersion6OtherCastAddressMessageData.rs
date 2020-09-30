// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Internet Protocol version 6 other cast (multicast or anycast) address message data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetInternetProtocolVersion6OtherCastAddressMessageData
{
	/// Common fields.
	#[serde(flatten)] pub common: GetAddressMessageDataCommon,

	/// Interface flags.
	pub interface_flags: InterfaceFlags,
	
	/// Multicast or Anycast address.
	pub other_cast_address: Option<in6_addr>
}
