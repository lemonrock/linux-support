// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessor;

impl<'cache> QueryProcessor<'cache> for MXQueryProcessor
{
	const DT: DataType = DataType::MX;
	
	type Record = EfficientCaseFoldedName;
	
	type RRV<'message> where 'cache: 'message = MXQueryProcessorResourceRecordVisitor<'cache, 'message>;
	
	fn new<'message>(query_name: &'message EfficientCaseFoldedName) -> Self::RRV<'message>
	where 'cache: 'message
	{
		MXQueryProcessorResourceRecordVisitor
		{
			query_name,
			present: PresentMultiple::default(),
		}
	}
	
	#[inline(always)]
	fn answered<'message>(finished: <<Self as QueryProcessor<'cache>>::RRV<'message> as ResourceRecordVisitor<'message>>::Finished, query_name: &'message EfficientCaseFoldedName, cache: &mut Cache<'cache>)
	where 'cache: 'message
	{
		cache.mx_query_type_cache.put_present_all_the_same_name(query_name.clone(), finished)
	}
}
