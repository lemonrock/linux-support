// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct QueryTypesCache
{
	pub(crate) A: QueryTypeCache<MultipleSortedRecords<Ipv4Addr>>,
	
	pub(crate) NS: QueryTypeCache<MultipleSortedRecords<DomainTarget>>,
	
	pub(crate) SOA: QueryTypeCache<StartOfAuthority<DomainTarget>>,
	
	pub(crate) AAAA: QueryTypeCache<MultipleSortedRecords<Ipv6Addr>>,
	
	pub(crate) MX: QueryTypeCache<MultiplePrioritizedThenSortedRecords<MailServerName<DomainTarget>>>,
	
	pub(crate) HINFO: QueryTypeCache<MultipleSortedRecords<HostInformation<OwnedCharacterString>>>,
	
	// pub(crate) SRV: QueryTypeCache<MultiplePrioritizedThenWeightedRecords<ServiceLocation<DomainTarget>>>,
	//
	// pub(crate) NAPTR: QueryTypeCache<MultipleOrderedThenPrioritizedThenUnsortedRecords<NamingAuthorityPointer<DomainTarget, OwnedUri, OwnedCharacterString, OwnedTypeEquality>>>,
}

impl Default for QueryTypesCache
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			A: QueryTypeCache::default(),
			NS: QueryTypeCache::default(),
			SOA: QueryTypeCache::default(),
			AAAA: QueryTypeCache::default(),
			MX: QueryTypeCache::default(),
			HINFO: QueryTypeCache::default(),
			// SRV: QueryTypeCache::default(),
			// NAPTR: QueryTypeCache::default(),
		}
	}
}
