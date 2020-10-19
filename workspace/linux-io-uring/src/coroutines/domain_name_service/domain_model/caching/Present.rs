// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
pub(crate) struct Present<Record: Sized + Clone>
{
	uncached: Vec<Record>,
	
	cached: BTreeMap<NanosecondsSinceUnixEpoch, Vec<Record>>,
}

impl<Record: Sized + Clone> Present<Record>
{
	#[inline(always)]
	fn records_count(&self) -> usize
	{
		let mut records_count = self.uncached.len();
		
		for records in self.cached.values()
		{
			records_count += records.len();
		}
		
		records_count
	}
	
	#[inline(always)]
	pub(crate) fn store<'message>(records: &mut HashMap<WithCompressionParsedName<'message>, Self>, name: WithCompressionParsedName<'message>, cache_until: CacheUntil, record: Record)
	{
		let present = records.entry(name).or_insert_with(|| Present::default());
		
		match cache_until
		{
			None => present.uncached.push(record),
			
			Some(cache_until) =>
			{
				let cached = present.cached.entry(cache_until).or_insert_with(|| Vec::new());
				cached.push(record)
			},
		}
		
	}
}
