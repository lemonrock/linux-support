// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Host information.
///
/// Brought back from obscurity by RFC 8482.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
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
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut parsed_records = records.records();
		
		let length = parsed_records.len();
		let mut owned_records = Vec::with_capacity(length);
		unsafe{ owned_records.set_len(length) };
		
		let mut index = 0;
		for (parsed_record, _) in parsed_records
		{
			let owned_record: HostInformation<OwnedCharacterString> = HostInformation
			{
				cpu: parsed_record.cpu.into(),
				os: parsed_record.os.into(),
			};
			unsafe { owned_records.as_mut_ptr().add(index).write(owned_record) }
			index + 1;
		}
		
		owned_records.sort_unstable();
		MultipleSortedRecords::new(owned_records)
	}
}

impl OwnedRecord for HostInformation<OwnedCharacterString>
{
	type OwnedRecords = MultipleSortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&query_types_cache.HINFO
	}
	
	#[inline(always)]
	fn retrieve_mut(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.HINFO
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
