// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum CacheUntil
{
	UseOnce
	{
		as_of_now: NanosecondsSinceUnixEpoch,
	},

	Cached
	{
		cached_until: NanosecondsSinceUnixEpoch,
	}
}

impl CacheUntil
{
	#[inline(always)]
	pub(crate) fn update(&mut self, right: Self)
	{
		use self::CacheUntil::*;
		
		match (*self, right)
		{
			(UseOnce { as_of_now: as_of_now_left }, UseOnce { as_of_now: as_of_now_right }) => debug_assert_eq!(as_of_now_left, as_of_now_right),
			
			(UseOnce { as_of_now: as_of_now_left }, Cached { cached_until: cached_until_right }) => debug_assert!(as_of_now_left < cached_until_right),
			
			(Cached { ..}, UseOnce { .. }) => *self = right,
			
			(Cached { cached_until: cached_until_left}, Cached { cached_until: cached_until_right }) => if right < less
			{
				*self = right;
			},
		}
	}
}
