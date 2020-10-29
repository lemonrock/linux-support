// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait QueryProcessor<'cache>
{
	const DT: DataType;
	
	type Record: Sized + Debug;
	
	type RRV<'message>: ResourceRecordVisitor<'message, Error=Infallible, Finished=Present<Self::Record>>
	where 'cache: 'message;
	
	fn new<'message>(query_name: &'message CaseFoldedName<'cache>) -> Self::RRV<'message>
	where 'cache: 'message;
	
	fn answered<'message>(finished: <<Self as QueryProcessor<'cache>>::RRV<'message> as ResourceRecordVisitor<'message>>::Finished, query_name: &'message CaseFoldedName<'cache>, cache: &mut Cache<'cache>)
	where 'cache: 'message;
	
	#[inline(always)]
	fn result<'message>(query_name: &'message CaseFoldedName<'cache>, cache: &mut Cache<'cache>, answer: Answer<'cache>, canonical_name_chain_records: Records<'cache, CaseFoldedName<'cache>>, finished: <<Self as QueryProcessor<'cache>>::RRV<'message> as ResourceRecordVisitor<'message>>::Finished)
	where 'cache: 'message
	{
		// TODO: This should update the no_domain_cache, as the domains now do exist!
		cache.cname_query_type_cache.put_present(canonical_name_chain_records);
		
		use self::Answer::*;
		use self::NoDomainResponseType::*;
		
		match answer
		{
			Answered =>
			{
				debug_assert!(!finished.is_empty());
				
				Self::answered(finished, query_name, cache);
			}
			
			// Be careful here - all the records of the cname chain bar most_canonical_name do exist, and so domain presence needs to be updated for them!!!
			NoDomain { response_type, most_canonical_name } =>
			{
				debug_assert!(finished.is_empty());
				
				match response_type
				{
				
				}
				
				//cache.no_domain_cache.put(query_name.clone(), )
			}
			
			NoData { response_type, most_canonical_name } =>
			{
				debug_assert!(finished.is_empty());
			
			}
			
			Referral { referral, most_canonical_name } =>
			{
				debug_assert!(finished.is_empty());
			
			}
		}
	}
}
