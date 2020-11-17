// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A mail server name.
#[derive(Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MailServerName<N: Name>(pub N);

impl<'message> Into<MailServerName<EfficientCaseFoldedName>> for MailServerName<ParsedName<'message>>
{
	#[inline(always)]
	fn into(self) -> MailServerName<EfficientCaseFoldedName>
	{
		MailServerName::new(EfficientCaseFoldedName::from(self.0))
	}
}

impl<'message> ParsedRecord for MailServerName<ParsedName<'message>>
{
	type OrderPriorityAndWeight = Priority;
	
	type OwnedRecord = MailServerName<EfficientCaseFoldedName>;
	
	#[inline(always)]
	fn into_owned_record(self) -> Self::OwnedRecord
	{
		MailServerName::new(EfficientCaseFoldedName::from(self.0))
	}
	
	#[inline(always)]
	fn store(cache: &mut QueryTypesCache, records: OwnerNameToRecordsValue<Self>)
	{
		cache.MX = QueryTypeCache::data(records.cache_until(), records.into());
	}
	
	#[inline(always)]
	fn no_data(cache: &mut QueryTypesCache, negative_cache_until: NegativeCacheUntil)
	{
		cache.MX = QueryTypeCache::no_data(negative_cache_until);
	}
}

impl<N: Name> MailServerName<N>
{
	#[inline(always)]
	pub(crate) const fn new(name: N) -> Self
	{
		Self(name)
	}
}
