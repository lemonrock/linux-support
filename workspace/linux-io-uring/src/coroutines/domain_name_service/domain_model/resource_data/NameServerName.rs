// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A name server name.
#[derive(Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NameServerName<N: Name>(pub N);

impl<'message> ParsedRecord for NameServerName<ParsedName<'message>>
{
	type OrderPriorityAndWeight = Priority;
	
	type OwnedRecord = NameServerName<EfficientCaseFoldedName>;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		NameServerName::new(EfficientCaseFoldedName::from(self.0))
	}
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		MultipleSortedRecords::<NameServerName<DomainTarget>>::from(records)
	}
}

impl OwnedRecord for NameServerName<EfficientCaseFoldedName>
{
	type OwnedRecords = MultipleSortedRecords<Self>;
	
	#[inline(always)]
	fn retrieve(query_types_cache: &mut QueryTypesCache) -> &mut Option<QueryTypeCache<Self::OwnedRecords>>
	{
		&mut query_types_cache.NS
	}
}

impl<N: Name> NameServerName<N>
{
	#[inline(always)]
	pub(crate) const fn new(name: N) -> Self
	{
		Self(name)
	}
}
