// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct DelegationNames(HashMap<EfficientCaseFoldedName, (CacheUntil, EfficientCaseFoldedName)>);

impl DelegationNames
{
	#[inline(always)]
	pub(crate) fn with_capacity(capacity: usize) -> Self
	{
		Self(HashMap::with_capacity(capacity))
	}
	
	#[inline(always)]
	pub(crate) fn store<'message>(&mut self, from: &ParsedName<'message>, cache_until: CacheUntil, to: &ParsedName<'message>) -> Result<(), CanonicalChainError>
	{
		let old = self.0.insert(EfficientCaseFoldedName::from(from), (cache_until, EfficientCaseFoldedName::from(to)));
		if unlikely!(old.is_some())
		{
			Err(CanonicalChainError::MoreThanOneDNAMEWithTheSameOwnerName)
		}
		else
		{
			Ok(())
		}
	}
}
