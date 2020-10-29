// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
enum NoDomainCacheEntry
{
	AbsentUseOnce,
	
	Present(NanosecondsSinceUnixEpoch),
}

impl From<NegativeCacheUntil> for NoDomainCacheEntry
{
	fn from(value: NegativeCacheUntil) -> Self
	{
		use self::NoDomainCacheEntry::*;
		
		match value
		{
			None => AbsentUseOnce,
			Some(cache_until) => Present(cache_until),
		}
	}
}

impl LeastRecentlyUsedCacheValue for NoDomainCacheEntry
{
	#[inline(always)]
	fn records_count(&self) -> NonZeroUsize
	{
		unsafe { NonZeroUsize::new_unchecked(1) }
	}
}
