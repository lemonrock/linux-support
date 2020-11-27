// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[serde(Deserialize, Serialize)]
pub(crate) struct QueryTypesCache
{
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
	fn is_empty(&self) -> bool
	{
		self.A.is_none()
			&& self.NS.is_none()
			&& self.SOA.is_none()
			&& self.MX.is_none()
			&& self.HINFO.is_none()
			&& self.TXT.is_none()
			&& self.AAAA.is_none()
			&& self.LOC_version_0.is_none()
			&& self.SRV.is_none()
			&& self.NAPTR.is_none()
	}
	
	pub(crate) fn for_ipv4only_arpa() -> Self
	{
		Self
		{
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
