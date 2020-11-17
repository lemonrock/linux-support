// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct QueryTypesCache
{
	NS: Option<MultipleSortedRecords<NameServerName<DomainTarget>>>,
	
	SOA: Option<SolitaryRecords<StartOfAuthority<DomainTarget>>>,
	
	A: Option<MultipleSortedRecords<Ipv4Addr>>,
	
	AAAA: Option<MultipleSortedRecords<Ipv6Addr>>,
	
	MX: Option<MultiplePrioritizedThenSortedRecords<MailServerName<DomainTarget>>>,
}

impl Default for QueryTypesCache
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			NS: None,
			SOA: None,
			A: None,
			AAAA: None,
			MX: None
		}
	}
}

impl QueryTypesCache
{
	#[inline(always)]
	pub(crate) fn store_A(&mut self, records: OwnerNameToRecordValue<Ipv4Addr>)
	{
		self.A = Some(records.into())
	}
	
	#[inline(always)]
	pub(crate) fn store_AAAA(&mut self, records: OwnerNameToRecordValue<Ipv6Addr>)
	{
		self.AAAA = Some(records.into())
	}
	
	#[inline(always)]
	pub(crate) fn store_MX<'message>(&mut self, records: OwnerNameToRecordValue<MailServerName<ParsedName<'message>>>)
	{
		self.MX = Some(records.into())
	}
}
