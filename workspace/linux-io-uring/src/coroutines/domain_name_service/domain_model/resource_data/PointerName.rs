// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A pointer name.
#[derive(Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct PointerName<N: Name>(pub N);

impl<'message> ParsedRecord for PointerName<ParsedName<'message>>
{
	type OrderPriorityAndWeight = Priority;
	
	type OwnedRecord = PointerName<DomainTarget>;
	
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
			let owned_record: PointerName<DomainTarget> = PointerName::new(AliasOrDomainTarget::from(parsed_record.0));
			unsafe { owned_records.as_mut_ptr().add(index).write(owned_record) }
			index + 1;
		}
		
		owned_records.sort_unstable();
		MultipleSortedRecords::new(owned_records)
	}
}

impl OwnedRecord for PointerName<DomainTarget>
{
	const SubdomainsAreNeverValid: bool = true;
	
	type OwnedRecords = MultipleSortedRecords<PointerName<DomainTarget>>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&query_types_cache.PTR
	}
	
	#[inline(always)]
	fn retrieve_mut(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.PTR
	}
	
	#[inline(always)]
	fn retrieve_fixed(query_types_fixed: &QueryTypesFixed) -> Option<&Self::OwnedRecords>
	{
		query_types_fixed.PTR.as_ref()
	}
}

impl<N: Name> PointerName<N>
{
	#[inline(always)]
	pub(crate) const fn new(name: N) -> Self
	{
		Self(name)
	}
}
