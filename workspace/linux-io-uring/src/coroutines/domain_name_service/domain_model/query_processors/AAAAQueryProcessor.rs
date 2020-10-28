// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AAAAQueryProcessor<'cache>
{
	records: Records<'cache, Ipv6Addr>,
}

impl<'cache> QueryProcessor<'cache> for AAAAQueryProcessor<'cache>
{
	const DT: DataType = DataType::A;
	
	type Record = Ipv6Addr;
	
	type RRV<'message> where 'cache: 'message = AAAAQueryProcessorResourceRecordVisitor<'cache>;
	
	fn new<'message>() -> Self::RRV<'message>
	where 'cache: 'message
	{
		AAAAQueryProcessorResourceRecordVisitor
		{
			records: Records::with_capacity(8),
		}
	}
	
	#[inline(always)]
	fn finish<'message>(finished: <<Self as QueryProcessor<'cache>>::RRV<'message> as ResourceRecordVisitor<'message>>::Finished, cache: &mut Cache<'cache>)
	where 'cache: 'message
	{
		cache.aaaa_query_type_cache.put_present(finished)
	}
}
