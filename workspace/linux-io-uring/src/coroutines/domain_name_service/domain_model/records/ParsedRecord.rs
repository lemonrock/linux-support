// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parsed record.
pub(crate) trait ParsedRecord: Sized + Debug
{
	/// Any Order, Priority or Weight (as a tuple).
	type OrderPriorityAndWeight: Sized;
	
	type OwnedRecord: OwnedRecord;
	
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords;
	
	#[inline(always)]
	fn store(subdomains_are_never_valid: NonNull<bool>, query_types_cache: &mut QueryTypesCache, records: OwnerNameToParsedRecordsValue<Self>)
	{
		Self::OwnedRecord::store(subdomains_are_never_valid, query_types_cache, Self::into_owned_records(records))
	}
	
	#[inline(always)]
	fn no_data(subdomains_are_never_valid: NonNull<bool>, query_types_cache: &mut QueryTypesCache, negative_cache_until: NegativeCacheUntil)
	{
		Self::OwnedRecord::no_data(subdomains_are_never_valid, query_types_cache, negative_cache_until)
	}
}

impl ParsedRecord for Ipv4Addr
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Self;
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut owned_records: Vec<Self> = unsafe { transmute(records.records()) };
		owned_records.sort_unstable();
		
		MultipleSortedRecords::new(owned_records)
	}
}

impl ParsedRecord for Ipv6Addr
{
	type OrderPriorityAndWeight = ();
	
	type OwnedRecord = Self;
	
	#[inline(always)]
	fn into_owned_records(records: OwnerNameToParsedRecordsValue<Self>) -> <Self::OwnedRecord as OwnedRecord>::OwnedRecords
	{
		let mut owned_records: Vec<Self> = unsafe { transmute(records.records()) };
		owned_records.sort_unstable();
		
		MultipleSortedRecords::new(owned_records)
	}
}
