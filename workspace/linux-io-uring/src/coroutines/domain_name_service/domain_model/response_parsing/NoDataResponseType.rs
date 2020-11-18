// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 2308, Section 3 Negative Answers from Authoritative Servers: "Name servers authoritative for a zone MUST include the SOA record of the zone in the authority section of the response when … indicating that no data of the requested type exists".
/// and
/// RFC 2308 Section 5: "Negative responses without SOA records SHOULD NOT be cached as there is no way to prevent the negative responses looping forever between a pair of servers even with a short TTL".
#[derive(Debug, Clone)]
pub(crate) enum NoDataResponseType
{
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1.
	NoDataResponseType1(AuthorityNameStartOfAuthorityNameServers),
	
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2.
	///
	/// RFC 2308, Section 2.2.1 Special Handling of No Data, Paragraph 1: "… it is recommended that servers that are authoritative for the NODATA response only send TYPE 2 NODATA responses, …".
	NoDataResponseType2(AuthorityNameStartOfAuthority),
	
	/// RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
	NoDataResponseType3
	{
		as_of_now: NanosecondsSinceUnixEpoch,
	},
}
