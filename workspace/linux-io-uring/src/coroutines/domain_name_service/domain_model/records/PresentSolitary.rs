// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum PresentSolitary<Record: Sized + Debug>
{
	/// One-time use.
	UseOnce
	{
		as_of_now: NanosecondsSinceUnixEpoch,
		
		record: Record,
	},

	/// Cached.
	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
		
		record: Record,
	}
}

impl<Record: Sized + Debug> PresentSolitary<Record>
{
	#[inline(always)]
	pub(crate) fn new(cache_until: CacheUntil, record: Record) -> Self
	{
		use self::PresentSolitary::*;
		
		match cache_until
		{
			CacheUntil::UseOnce { as_of_now } => UseOnce { as_of_now, record },
			
			CacheUntil::Cached { cached_until } => Cached { cached_until, record }
		}
	}
}