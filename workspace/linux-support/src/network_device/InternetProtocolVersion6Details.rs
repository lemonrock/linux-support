// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InternetProtocolVersion6Details
{
	#[serde(flatten)] pub device_configuration: InternetProtocolVersion6DeviceConfiguration,
	
	pub flags: u32,
	
	pub cache_timestamp_relative_to_boot_time: Milliseconds,
	
	pub cache_reachable_time: Milliseconds,
	
	pub cache_retransmit_time: Milliseconds,
	
	pub statistics: Box<[u64]>,
	
	pub icmpv6_statistics: Box<[u64]>,
	
	pub token: in6_addr,
	
	pub address_generation_mode: in6_addr_gen_mode,
}
