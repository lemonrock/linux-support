// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AAAAQueryProcessor<'cache>
{
	records: Records<'cache, Ipv6Addr>,
}

impl<'message, 'cache: 'message> ResourceRecordVisitor<'message> for AAAAQueryProcessor<'cache>
{
	type Error = Infallible;
	
	type Finished = Records<'cache, Ipv6Addr>;
	
	#[inline(always)]
	fn finished(self) -> Self::Finished
	{
		self.records
	}
	
	#[inline(always)]
	fn AAAA(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: Ipv6Addr) -> Result<(), Self::Error>
	{
		self.records.store_unprioritized_and_unweighted(name, cache_until, record);
		Ok(())
	}
}

impl<'message, 'cache: 'message> QueryProcessor<'message, 'cache> for AAAAQueryProcessor<'cache>
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
	fn finish(finished: <Self as ResourceRecordVisitor<'message>>::Finished, cache: &mut Cache<'cache>)
	{
		cache.aaaa_query_type_cache.put_present(finished)
	}
}
