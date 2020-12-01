// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `TXT` record.
#[derive(Debug, Clone)]
pub struct Text<CS: CharacterString>(Vec<CS>);

impl<'message> ParsedRecord for Text<ParsedCharacterString<'message>>
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Text<OwnedCharacterString>;
	
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
			let owned_record = parsed_record.into_owned_record();
			unsafe { owned_records.as_mut_ptr().add(index).write(owned_record) }
			index + 1;
		}
		
		owned_records.sort_unstable();
		MultipleUnsortedRecords::new(owned_records)
	}
}

impl OwnedRecord for Text<OwnedCharacterString>
{
	type OwnedRecords = MultipleUnsortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&query_types_cache.TXT
	}
	
	#[inline(always)]
	fn retrieve_mut(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.TXT
	}
}

impl<'message> Text<ParsedCharacterString<'message>>
{
	#[inline(always)]
	pub(crate) const fn new(strings: Vec<ParsedCharacterString<'message>>) -> Self
	{
		Self(strings)
	}
	
	#[inline(always)]
	fn into_owned_record(self) -> Text<OwnedCharacterString>
	{
		let length = self.0.len();
		let mut owned_character_strings = Vec::with_capacity(length);
		unsafe{ owned_character_strings.set_len(length) };
		
		let mut index = 0;
		for parsed_character_string in self.0
		{
			let owned_character_string = OwnedCharacterString::from(parsed_character_string);
			unsafe { owned_character_strings.as_mut_ptr().add(index).write(owned_character_string) }
			index + 1;
		}
		
		Text(owned_character_strings)
	}
}

impl Deref for Text<OwnedCharacterString>
{
	type Target = [OwnedCharacterString];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}
