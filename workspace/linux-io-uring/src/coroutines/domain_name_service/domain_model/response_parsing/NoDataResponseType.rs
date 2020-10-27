// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NoDataResponseType<'label, N: Name<'label>>
{
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1.
	NoDataResponseType1
	{
		start_of_authority: (CaseFoldedName<'static>, NegativeCacheUntil, StartOfAuthority<'label, N>),
		
		name_servers: Records<'label, N>,
	},
	
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2.
	NoDataResponseType2
	{
		start_of_authority: (CaseFoldedName<'static>, NegativeCacheUntil, StartOfAuthority<'label, N>),
	},
	
	// TODO: RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
	NoDataResponseType3
	{
		name_servers: Records<'label, N>,
	},
}

impl<'label, N: Name<'label>> NoDataResponseType<'label, N>
{
	/// RFC 2308, Section 6, Negative answers from the cache: "`NXDOMAIN` types 1 and 4 responses contain implicit referrals as does `NODATA` type 1 response".
	#[inline(always)]
	fn is_implicit_referral(&self) -> bool
	{
		use self::NoDataResponseType::*;
		
		match self
		{
			&NoDataResponseType1 { .. } => true,
			
			&NoDataResponseType2 { .. } | &NoDataResponseType3 { .. } => false,
		}
	}
}
