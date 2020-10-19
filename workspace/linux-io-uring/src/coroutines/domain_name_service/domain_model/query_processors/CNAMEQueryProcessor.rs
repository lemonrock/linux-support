// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default)]
struct CNAMEQueryProcessor<'message>
{
	records: Option<Record<NameAsLabelsIncludingRoot>>,
}

impl<'message, A: Alloctor> ResourceRecordVisitor<'message> for CNAMEQueryProcessor<A>
{
	#[inline(always)]
	fn CNAME(&mut self, name: WithCompressionParsedName<'message>, cache_until: CacheUntil, record: WithCompressionParsedName<'message>, is_some_if_present_in_answer_section_and_true_if_was_queried_for: Option<bool>) -> Result<(), DnsProtocolError>
	{
		debug_assert_eq!(is_some_if_present_in_answer_section_and_true_if_was_queried_for, Some(true));
		debug_assert!(self.records.is_none(), "This should have been checked for in `ResourceRecord::parse_answer_section_resource_record_in_response()`");
		
		self.records = Some(Record::new_from_parsed_data(name, time_to_live, record));
		
		Ok(())
	}
}

impl<'message, A: Allocator> QueryProcessor<'message, NameAsLabelsIncludingRoot> for CNAMEQueryProcessor<A>
{
	const DT: DataType = DataType::CNAME;
	
	type R = Option<Record<UncompressedName<A>, A>>;
	
	#[inline(always)]
	fn finish(self) -> Self::R
	{
		self.records
	}
}
