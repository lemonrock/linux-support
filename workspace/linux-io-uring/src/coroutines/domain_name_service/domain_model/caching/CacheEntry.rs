// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) enum CacheEntry<Record: Sized + Clone>
{
	AbsentButUncached,
	
	AbsentNegativelyCached(NanosecondsSinceUnixEpoch),
	
	Present(Present<Record>),
}

impl<Record: Sized + Clone> CacheEntry<Record>
{
	#[inline(always)]
	pub(crate) fn records_count(&self) -> usize
	{
		use self::CacheEntry::*;
		
		match self
		{
			AbsentButUncached | AbsentNegativelyCached(_) => 1,
			
			Present(ref present) => present.records_count(),
		}
	}
}
