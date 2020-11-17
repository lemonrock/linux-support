// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct MXQueryProcessor;

impl QueryProcessor for MXQueryProcessor
{
	const DT: DataType = DataType::MX;
	
	type PR<'message> = MailServerName<ParsedName<'message>>;
	
	type RRV<'message> = MXQueryProcessorResourceRecordVisitor<'message>;
	
	fn new<'message>(query_name: &'message EfficientCaseFoldedName) -> Self::RRV<'message>
	{
		MXQueryProcessorResourceRecordVisitor
		{
			query_name,
			records: OwnerNameToRecords::default(),
		}
	}
	
	#[inline(always)]
	fn store_records_in_query_types_cache<'message>(query_types_cache: &mut QueryTypesCache, records: OwnerNameToRecordValue<Self::PR<'message>>)
	{
		query_types_cache.store_MX(records)
	}
}
