// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
enum NoDomainResponseType<'label, N: Name<'label>>
{
	/// RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 1.
	NoDomainResponseType1
	{
		start_of_authority: (N, NegativeCacheUntil, StartOfAuthority<'label, N>),
		
		name_servers: Records<'label, N>,
	},
	
	/// RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 2.
	NoDomainResponseType2
	{
		start_of_authority: (N, NegativeCacheUntil, StartOfAuthority<'label, N>),
	},
	
	// TODO: RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
	/// RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 3.
	NoDomainResponseType3,
	
	// TODO: RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
	/// RFC 2308, Section 2.1 Name Error NXDOMAIN RESPONSE: TYPE 4.
	NoDomainResponseType4
	{
		name_servers: Records<'label, N>,
	},
}

impl<'label, N: Name<'label>> NoDomainResponseType<'label, N>
{
	/// RFC 2308, Section 6, Negative answers from the cache: "`NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
	#[inline(always)]
	fn is_implicit_referral(&self) -> bool
	{
		use self::NoDomainResponseType::*;
		
		match self
		{
			&NoDomainResponseType1 { .. } | &NoDomainResponseType4 { .. } => true,
			
			&NoDomainResponseType2 { .. } | &NoDomainResponseType3 => false,
		}
	}
}
