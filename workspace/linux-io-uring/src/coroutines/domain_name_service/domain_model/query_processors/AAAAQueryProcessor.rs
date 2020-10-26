// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
struct AAAAQueryProcessor<'message>
{
	records: Records<'message, Ipv6Addr>,
}

impl<'message> ResourceRecordVisitor<'message> for AAAAQueryProcessor<'message>
{
	type Error = Infallible;
	
	#[inline(always)]
	fn AAAA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Ipv6Addr) -> Result<(), Self::Error>
	{
		self.records.store_unprioritized_and_unweighted::<'message>(name, cache_until, record);
		
		Ok(())
	}
}

impl<'message> QueryProcessor<'message> for AAAAQueryProcessor
{
	const DT: DataType = DataType::AAAA;
	
	type Record = Ipv6Addr;
	
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			records: Records::with_capacity(8),
		}
	}
	
	#[inline(always)]
	fn finish(self, cache: &mut QueryTypeCache<Self::Record>)
	{
		cache.put_present::<'message>(self.records)
	}
}
