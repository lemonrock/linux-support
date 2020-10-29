// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 2308, Section 3 Negative Answers from Authoritative Servers: "Name servers authoritative for a zone MUST include the SOA record of the zone in the authority section of the response when … indicating that no data of the requested type exists".
/// and
/// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
#[derive(Debug, Clone)]
pub(crate) enum NoDataResponseType<'label, N: Name<'label>>
{
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1.
	NoDataResponseType1
	{
		authority_name: N,
		
		start_of_authority: (NegativeCacheUntil, StartOfAuthority<'label, N>),
		
		/// These are for `authority_name`.
		name_servers: Present<N>,
	},
	
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2.
	///
	/// RFC 2308, Section 2.2.1 Special Handling of No Data, Paragraph 1: "… it is recommended that servers that are authoritative for the NODATA response only send TYPE 2 NODATA responses, …".
	NoDataResponseType2
	{
		authority_name: N,
		
		/// This is for `authority_name`.
		start_of_authority: (NegativeCacheUntil, StartOfAuthority<'label, N>),
	},
	
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
	NoDataResponseType3,
}

impl<'label, N: Name<'label>> NoDataResponseType<'label, N>
{
	/// RFC 2308, Section 4 SOA Minimum Field, Paragraph 6, Page 9: "…  being the TTL to be used for negative responses, is the new defined meaning of the SOA minimum field".
	#[inline(always)]
	fn negative_cache_until_for_negative_response(&self) -> NegativeCacheUntil
	{
		use self::NoDataResponseType::*;
		
		match self
		{
			&NoDataResponseType1 { start_of_authority: (negative_cache_until, _), .. } | &NoDataResponseType2 { start_of_authority: (negative_cache_until, _), .. } => negative_cache_until,
			
			&NoDataResponseType3 => None,
		}
	}
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
