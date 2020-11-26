// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[serde(Deserialize, Serialize)]
pub(crate) struct QueryTypesCache
{
	pub(crate) A: Option<QueryTypeCache<MultipleSortedRecords<Ipv4Addr>>>,
	
	pub(crate) NS: Option<QueryTypeCache<MultipleSortedRecords<NameServerName<DomainTarget>>>>,
	
	pub(crate) SOA: Option<QueryTypeCache<StartOfAuthority<DomainTarget>>>,
	
	pub(crate) AAAA: Option<QueryTypeCache<MultipleSortedRecords<Ipv6Addr>>>,
	
	pub(crate) MX: Option<QueryTypeCache<MultiplePrioritizedThenSortedRecords<MailServerName<DomainTarget>>>>,
	
	pub(crate) HINFO: Option<QueryTypeCache<MultipleSortedRecords<HostInformation<OwnedCharacterString>>>>,
	
	pub(crate) PTR: Option<QueryTypeCache<MultipleSortedRecords<PointerName<AliasOrDomainTarget>>>>,
	
	// pub(crate) SRV: Option<QueryTypeCache<MultiplePrioritizedThenWeightedRecords<ServiceLocation<DomainTarget>>>>,
	//
	// pub(crate) NAPTR: Option<QueryTypeCache<MultipleOrderedThenPrioritizedThenUnsortedRecords<NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>>>>,
}

impl Default for QueryTypesCache
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			A: None,
			
			NS: None,
			
			SOA: None,
			
			AAAA: None,
			
			MX: None,
			
			HINFO: None,
			
			PTR: None,
			
			// SRV: None,
			
			// NAPTR: None,
		}
	}
}

impl QueryTypesCache
{
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.A.is_none() && self.NS.is_none() && self.SOA.is_none() && self.AAAA.is_none() && self.MX.is_none() && self.HINFO.is_none()
	}
	
	pub(crate) fn for_ipv4only_arpa() -> Self
	{
		Self
		{
			A:
			{
				let mut records = MultipleSortedRecords::single(Ipv4Addr::new(192, 0, 0, 170));
				records.add(Ipv4Addr::new(192, 0, 0, 171));
				QueryTypeCache::data_forever(records)
			},
			
			NS: QueryTypeCache::no_data_forever(),
			
			SOA: QueryTypeCache::no_data_forever(),
			
			AAAA: QueryTypeCache::no_data_forever(),
			
			MX: QueryTypeCache::no_data_forever(),
			
			HINFO: QueryTypeCache::no_data_forever(),
			
			PTR: QueryTypeCache::no_data_forever(),
		}
	}
}
