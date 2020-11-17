// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


enum NoDataCacheEntry
{
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 when the TTL of the `SOA` record is 0;
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
	UseOnce
	{
		as_of_now: NanosecondsSinceUnixEpoch,
		
		/// Looking this up will return either nothing, a `StartOfAuthority` and/or name servers.
		///
		/// Is `None` for:-
		///
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 3.
		///
		/// Is `Some()` for:-
		///
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 (in which case the `authority_name` will point to `SOA` and `NS` records);
		/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 (in which case the `authority_name` will point to a `SOA` record).
		authority_name: Option<DomainTarget>,
	},
	
	/// Corresponds to the *end* (final target) of the canonical name chain for:-
	///
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 1 when the TTL is *not* 0 (in which case the `authority_name` will point to `SOA` and `NS` records).
	/// * RFC 2308, Section 2.2 No Data NODATA RESPONSE: TYPE 2 when the TTL is *not* 0 (in which case the `authority_name` will point to a `SOA` record).
	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
		
		authority_name: DomainTarget,
	}
}
