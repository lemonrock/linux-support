// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Host information.
///
/// Brought back from obscurity by RFC 8482.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostInformation<CS: CharacterString>
{
	/// `CPU` field.
	///
	/// In RFC 8482, this will be `RFC8482`.
	pub cpu: CS,

	/// `OS` field.
	///
	/// In RFC 8482, this will be ``.
	pub os: CS,
}

impl<'message> ParsedRecord for HostInformation<ParsedCharacterString<'message>>
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = HostInformation<OwnedCharacterString>;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		HostInformation
		{
			cpu: self.cpu.into(),
			os: self.os.into(),
		}
	}
	
	#[inline(always)]
	fn store(cache: &mut QueryTypesCache, records: OwnerNameToRecordsValue<Self>)
	{
		cache.HINFO = QueryTypeCache::data(records.cache_until(), records.into());
	}
	
	#[inline(always)]
	fn no_data(cache: &mut QueryTypesCache, negative_cache_until: NegativeCacheUntil)
	{
		cache.HINFO = QueryTypeCache::no_data(negative_cache_until);
	}
}

impl<CS: CharacterString> HostInformation<CS>
{
	/// Is this a RFC 8482 answer to the `ANY` / `*` `QTYPE` question?
	#[inline(always)]
	pub fn is_rfc_8482_answer_to_any_question(&self) -> bool
	{
		self.cpu.deref() == b"RFC8482" && self.os.is_empty()
	}

	/// Is this a CloudFlare answer to the `ANY` / `*` `QTYPE` question?
	#[inline(always)]
	pub fn is_cloudflare_answer_to_any_question(&self) -> bool
	{
		self.cpu.deref() == b"ANY obsoleted" && self.os == b"See draft-ietf-dnsop-refuse-any"
	}
}
