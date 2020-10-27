// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub trait QueryProcessor<'message, 'cache: 'message>: Sized + ResourceRecordVisitor<'message>
{
	const DT: DataType;
	
	type Record: Sized;
	
	fn new() -> Self;
	
	fn finish(finished: <Self as ResourceRecordVisitor<'message>>::Finished, cache: &mut Cache<'cache>);
	
	#[inline(always)]
	fn result(cache: &mut Cache<'cache>, answer: Answer<'cache, CaseFoldedName<'cache>>, canonical_name_chain_records: Records<'cache, CaseFoldedName<'cache>>, finished: <Self as ResourceRecordVisitor<'message>>::Finished) -> Result<(), ProtocolError<Self::Error>>
	{
		cache.cname_query_type_cache.put_present(canonical_name_chain_records);
		Self::finish(finished, cache);
		
		Ok(())
	}
}
