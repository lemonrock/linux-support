// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
struct CNAMEQueryProcessor<'message>
{
	records: Option<Record<CaseFoldedName>>,
}

impl<'message, A: Alloctor> ResourceRecordVisitor<'message> for CNAMEQueryProcessor<A>
{
	#[inline(always)]
	fn CNAME(&mut self, name: ParsedName<'message>, cache_until: CacheUntil, record: ParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), DnsProtocolError>
	{
		debug_assert_eq!(is_some_if_present_in_answer_section_and_true_if_was_queried_for, Some(true));
		debug_assert!(self.records.is_none(), "This should have been checked for in `ResourceRecord::parse_answer_section_resource_record_in_response()`");
		
		self.records = Some(Record::from(name, time_to_live, record));
		
		Ok(())
	}
}

impl<'message> QueryProcessor<'message, CaseFoldedName> for CNAMEQueryProcessor
{
	const DT: DataType = DataType::CNAME;
	
	type Record = ();
	
	fn new() -> Self {
		unimplemented!()
	}
	
	fn finish(self, cache: &mut QueryTypeCache<Self::Record>) {
		unimplemented!()
	}
}
