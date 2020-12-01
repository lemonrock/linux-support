// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[serde(Deserialize, Serialize)]
pub(crate) struct QueryTypesCache
{
	fields_that_are_some: u8,
	
	pub(crate) A: Option<QueryTypeCache<MultipleSortedRecords<Ipv4Addr>>>,
	
	pub(crate) NS: Option<QueryTypeCache<MultipleSortedRecords<NameServerName<DomainTarget>>>>,
	
	pub(crate) SOA: Option<QueryTypeCache<StartOfAuthority<DomainTarget>>>,
	
	pub(crate) MX: Option<QueryTypeCache<MultiplePrioritizedThenSortedRecords<MailServerName<DomainTarget>>>>,
	
	pub(crate) HINFO: Option<QueryTypeCache<MultipleSortedRecords<HostInformation<OwnedCharacterString>>>>,
	
	pub(crate) PTR: Option<QueryTypeCache<MultipleSortedRecords<PointerName<DomainTarget>>>>,
	
	pub(crate) TXT: Option<QueryTypeCache<MultipleUnsortedRecords<Text<OwnedCharacterString>>>>,
	
	pub(crate) AAAA: Option<QueryTypeCache<MultipleSortedRecords<Ipv6Addr>>>,
	
	pub(crate) LOC_version_0: Option<QueryTypeCache<MultipleUnsortedRecords<LocationBodyVersion0>>>,
	
	pub(crate) SRV: Option<QueryTypeCache<MultiplePrioritizedThenWeightedRecords<ServiceLocation<DomainTarget>>>>,
	
	pub(crate) NAPTR: Option<QueryTypeCache<MultipleOrderedThenPrioritizedThenUnsortedRecords<NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>>>>,
}

impl Default for QueryTypesCache
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			fields_that_are_some: 0,
			
			A: None,
			
			NS: None,
			
			SOA: None,
			
			MX: None,
			
			HINFO: None,
			
			PTR: None,
			
			TXT: None,
			
			AAAA: None,
			
			LOC_version_0: None,
			
			SRV: None,
			
			NAPTR: None,
		}
	}
}

impl QueryTypesCache
{
	#[inline(always)]
	pub(crate) fn store<ORs: OwnedRecords<OR>, OR: OwnedRecord>(&mut self, subdomains_are_never_valid: NonNull<bool>, records: ORs)
	{
		let query_type_cache = OR::retrieve_mut(self);
		let increment = query_type_cache.is_none() as u8;
		
		unsafe { * subdomains_are_never_valid.as_ptr() = OR::SubdomainsAreNeverValid };
		
		*query_type_cache = QueryTypeCache::data(records.cache_until(), records);
		self.fields_that_are_some += increment;
	}
	
	#[inline(always)]
	pub(crate) fn no_data<OR: OwnedRecord>(&mut self, subdomains_are_never_valid: NonNull<bool>, negative_cache_until: NegativeCacheUntil)
	{
		let query_type_cache = OR::retrieve_mut(self);
		let decrement = query_type_cache.is_some() as u8;
		
		unsafe { * subdomains_are_never_valid.as_ptr() = OR::SubdomainsAreNeverValid };
		
		*query_type_cache = QueryTypeCache::no_data(negative_cache_until);
		self.fields_that_are_some -= decrement;
	}
	
	#[inline(always)]
	fn expired(&mut self, query_types_cache_field: QueryTypesCacheField)
	{
		use self::QueryTypesCacheField::*;
		
		debug_assert_ne!(self.fields_that_are_some, 0);
		
		match query_types_cache_field
		{
			A => QueryTypeCache::expired(&mut self.A),
			
			NS => QueryTypeCache::expired(&mut self.NS),
			
			SOA => QueryTypeCache::expired(&mut self.SOA),
			
			MX => QueryTypeCache::expired(&mut self.MX),
			
			HINFO => QueryTypeCache::expired(&mut self.HINFO),
			
			PTR => QueryTypeCache::expired(&mut self.PTR),
			
			TXT => QueryTypeCache::expired(&mut self.TXT),
			
			AAAA => QueryTypeCache::expired(&mut self.AAAA),
			
			LOC_version_0 => QueryTypeCache::expired(&mut self.LOC_version_0),
			
			SRV => QueryTypeCache::expired(&mut self.SRV),
			
			NAPTR => QueryTypeCache::expired(&mut self.NAPTR),
		}
		
		self.fields_that_are_some -= 1;
	}
	
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.fields_that_are_some == 0
	}
	
	pub(crate) fn for_ipv4only_arpa() -> Self
	{
		Self
		{
			fields_that_are_some: 1,
			
			A:
			{
				let mut records = MultipleSortedRecords::single(Ipv4Addr::new(192, 0, 0, 170));
				records.add_inefficient(Ipv4Addr::new(192, 0, 0, 171));
				QueryTypeCache::data_forever(records)
			},
			
			NS: QueryTypeCache::no_data_forever(),
			
			SOA: QueryTypeCache::no_data_forever(),
			
			MX: QueryTypeCache::no_data_forever(),
			
			HINFO: QueryTypeCache::no_data_forever(),
			
			PTR: QueryTypeCache::no_data_forever(),
			
			TXT: QueryTypeCache::no_data_forever(),
			
			AAAA: QueryTypeCache::no_data_forever(),
			
			LOC_version_0: QueryTypeCache::no_data_forever(),
			
			SRV: QueryTypeCache::no_data_forever(),
			
			NAPTR: QueryTypeCache::no_data_forever(),
		}
	}
	
	/// As per RFC 8880, Section 7.2, Names '170.0.0.192.in‑addr.arpa' and '171.0.0.192.in‑addr.arpa'.
	///
	/// In practice, the domain `192.in-addr.arpa` reports `NXDOMAIN`.
	pub(crate) fn pointer_for_ipv4only_arpa() -> Self
	{
		Self
		{
			fields_that_are_some: 1,
			
			A: QueryTypeCache::no_data_forever(),
			
			NS: QueryTypeCache::no_data_forever(),
			
			SOA: QueryTypeCache::no_data_forever(),
			
			MX: QueryTypeCache::no_data_forever(),
			
			HINFO: QueryTypeCache::no_data_forever(),
			
			PTR: QueryTypeCache::data_forever(MultipleSortedRecords::single(PointerName::new(EfficientCaseFoldedName::second_level(EfficientCaseFoldedLabel::ipv4only, EfficientCaseFoldedLabel::arpa)))),
			
			TXT: QueryTypeCache::no_data_forever(),
			
			AAAA: QueryTypeCache::no_data_forever(),
			
			LOC_version_0: QueryTypeCache::no_data_forever(),
			
			SRV: QueryTypeCache::no_data_forever(),
			
			NAPTR: QueryTypeCache::no_data_forever(),
		}
	}
}
