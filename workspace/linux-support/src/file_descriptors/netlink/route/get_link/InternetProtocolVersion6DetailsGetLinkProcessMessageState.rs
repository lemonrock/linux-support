// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct InternetProtocolVersion6DetailsGetLinkProcessMessageState
{
	pub(crate) device_configuration: Option<InternetProtocolVersion6DeviceConfiguration>,
	
	pub(crate) flags: Option<u32>,
	
	pub(crate) cache_information: Option<ifla_cacheinfo>,
	
	pub(crate) statistics: Option<Box<[u64]>>,
	
	pub(crate) icmpv6_statistics: Option<Box<[u64]>>,
	
	pub(crate) token: Option<in6_addr>,
	
	pub(crate) address_generation_mode: Option<in6_addr_gen_mode>,
}

impl InternetProtocolVersion6DetailsGetLinkProcessMessageState
{
	#[inline(always)]
	pub(crate) fn to_processed_message(self) -> Result<InternetProtocolVersion6Details, String>
	{
		let cache_information = self.cache_information.ok_or(format!("Linux kernel bug - missing cache_information"))?;
		cache_information.debug_assert_max_reasm_len_is_constant();
		
		Ok
		(
			InternetProtocolVersion6Details
			{
				device_configuration: self.device_configuration.ok_or(format!("Linux kernel bug - missing device_configuration"))?,
				
				flags: self.flags.ok_or(format!("Linux kernel bug - missing flags"))?,
				
				cache_timestamp_relative_to_boot_time: Milliseconds(cache_information.tstamp),
				
				cache_reachable_time: Milliseconds(cache_information.reachable_time),
				
				cache_retransmit_time: Milliseconds(cache_information.retrans_time),
				
				statistics: self.statistics.ok_or(format!("Linux kernel bug - missing statistics"))?,
				
				icmpv6_statistics: self.icmpv6_statistics.ok_or(format!("Linux kernel bug - missing icmpv6_statistics"))?,
				
				token: self.token.ok_or(format!("Linux kernel bug - missing token"))?,
				
				address_generation_mode: self.address_generation_mode.ok_or(format!("Linux kernel bug - missing address_generation_mode"))?,
			}
		)
	}
}
