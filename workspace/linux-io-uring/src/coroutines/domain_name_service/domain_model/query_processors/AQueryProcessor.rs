// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// TODO: MX, CNAME, CNAME chains caching

#[derive(Default)]
struct AQueryProcessor<'message>
{
	records: HashMap<WithCompressionParsedName<'message>, Present<Ipv4Addr>>
}

impl<'message> ResourceRecordVisitor<'message> for AQueryProcessor<'message>
{
	#[inline(always)]
	fn A(&mut self, name: WithCompressionParsedName<'message>, cache_until: CacheUntil, record: Ipv4Addr) -> Result<(), DnsProtocolError>
	{
		Present::store::<'message>(&mut self.records,name, cache_until, record);
		
		Ok(())
	}
}

impl<'message> QueryProcessor<'message, Ipv4Addr> for AQueryProcessor
{
	const DT: DataType = DataType::A;
	
	type Record = Ipv4Addr;
	
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			records: HashMap::with_capacity(8),
		}
	}
	
	#[inline(always)]
	fn finish(self, cache: &mut QueryTypeCache<Self::Record>)
	{
		cache.put_present::<'message>(self.records)
	}
}
