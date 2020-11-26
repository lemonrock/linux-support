// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum FixedDomainCacheEntry
{
	/// `A` and `AAAA` records.
	///
	/// Used for records in, say, `/etc/hosts`.
	QueryTypesFixed(QueryTypesFixed),

	/// An alias.
	///
	/// Used for aliases in, say, `/etc/hosts`.
	Alias(DomainTarget),
}

impl FixedDomainCacheEntry
{
	#[inline(always)]
	pub(crate) fn new_local_internet_protocol_addresses() -> Self
	{
		FixedDomainCacheEntry::QueryTypesFixed(QueryTypesFixed::new_local_internet_protocol_addresses())
	}
	
	#[inline(always)]
	pub(crate) fn query_types_fixed(internet_protocol_address: IpAddr) -> Self
	{
		FixedDomainCacheEntry::QueryTypesFixed(QueryTypesFixed::new(internet_protocol_address))
	}
	
	#[inline(always)]
	pub(crate) fn alias(canonical_name: &DomainTarget) -> Self
	{
		FixedDomainCacheEntry::Alias(canonical_name.clone())
	}
}
