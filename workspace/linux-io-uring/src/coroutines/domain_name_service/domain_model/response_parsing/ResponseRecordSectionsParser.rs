// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct ResponseRecordSectionsParser<'message>
{
	now: NanosecondsSinceUnixEpoch,
	data_type: DataType,
	end_of_message_pointer: usize,
	message_header: &'message MessageHeader,
	
	parsed_names: RefCell<ParsedNames>,
	response_parsing_state: ResponseParsingState,
	duplicate_resource_record_response_parsing: DuplicateResourceRecordResponseParsing<'message>,
}

impl<'message> ResponseRecordSectionsParser<'message>
{
	#[inline(always)]
	pub(crate) fn new(now: NanosecondsSinceUnixEpoch, data_type: DataType, end_of_message_pointer: usize, message_header: &'message MessageHeader, parsed_names: ParsedNames) -> Self
	{
		Self
		{
			now,
			data_type,
			end_of_message_pointer,
			message_header,
			
			parsed_names: RefCell::new(parsed_names),
			response_parsing_state: ResponseParsingState::new(),
			duplicate_resource_record_response_parsing: DuplicateResourceRecordResponseParsing::default(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_answer_authority_and_additional_sections<RRV: ResourceRecordVisitor<'message>>(&mut self, next_resource_record_pointer: usize, query_name: ParsedName<'message>, answer_quality: AnswerQuality, answer_section_resource_record_visitor: &mut RRV) -> Result<(usize, AnswerOutcome, CanonicalNameChain<'message>), SectionError<RRV::Error>>
	{
		let (next_resource_record_pointer, canonical_name_chain, answer_section_has_at_least_one_record_of_requested_data_type) = self.parse_answer_section(next_resource_record_pointer, query_name, answer_section_resource_record_visitor)?;

		let (next_resource_record_pointer, answer_outcome, canonical_name_chain) = self.parse_authority_section(next_resource_record_pointer, canonical_name_chain, answer_quality, answer_section_has_at_least_one_record_of_requested_data_type)?;

		let next_resource_record_pointer = self.parse_additional_section(next_resource_record_pointer)?;

		Ok((next_resource_record_pointer, answer_outcome, canonical_name_chain))
	}

	#[inline(always)]
	fn parse_answer_section<RRV: ResourceRecordVisitor<'message>>(&mut self, next_resource_record_pointer: usize, query_name: ParsedName<'message>, answer_section_resource_record_visitor: &mut RRV) -> Result<(usize, CanonicalNameChain<'message>, bool), AnswerSectionError<WrappingCanonicalChainError<RRV::Error>>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_answer_section();

		let mut resource_record_visitor = CanonicalNameChainAnswerSectionResourceRecordVisitor::new(answer_section_resource_record_visitor, query_name);
		let mut answer_section_has_at_least_one_record_of_requested_data_type = true;
		
		let parse_method = |resource_record: ResourceRecord| resource_record.parse_answer_section_resource_record_in_response(self.now, self.data_type, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing, &mut answer_section_has_at_least_one_record_of_requested_data_type);
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AnswerSectionError::ResourceRecordsOverflowSection, parse_method)?;
		
		Ok((next_resource_record_pointer, resource_record_visitor.into(), answer_section_has_at_least_one_record_of_requested_data_type))
	}

	#[inline(always)]
	fn parse_authority_section(&mut self, next_resource_record_pointer: usize, canonical_name_chain: CanonicalNameChain<'message>, answer_quality: AnswerQuality, answer_section_has_at_least_one_record_of_requested_data_type: bool) -> Result<(usize, AnswerOutcome, CanonicalNameChain<'message>), AuthoritySectionError<AuthorityError>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_authority_records_section();

		let mut authority_resource_record_visitor = AuthorityResourceRecordVisitor::new(canonical_name_chain);
		
		let parse_method = |resource_record: ResourceRecord| resource_record.parse_authority_section_resource_record_in_response(self.now, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut authority_resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing);
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AuthoritySectionError::ResourceRecordsOverflowSection, parse_method)?;

		let (answer_outcome, canonical_name_chain) = authority_resource_record_visitor.answer_outcome(answer_quality.is_authoritative_answer(), answer_quality.has_nxdomain_error_code(), answer_section_has_at_least_one_record_of_requested_data_type)?;

		Ok((next_resource_record_pointer, answer_outcome, canonical_name_chain))
	}

	#[inline(always)]
	fn parse_additional_section(&mut self, next_resource_record_pointer: usize) -> Result<usize, AdditionalSectionError<Infallible>>
	{
		let number_of_resource_records = self.message_header.number_of_resource_records_in_the_additional_records_section();
		
		let mut discarding_resource_record_visitor = DiscardingResourceRecordVisitor::default();
		
		let parse_method = |resource_record: ResourceRecord| resource_record.parse_additional_section_resource_record_in_response(self.now, self.end_of_message_pointer, self.parsed_names.borrow_mut().deref_mut(), &mut discarding_resource_record_visitor, &self.response_parsing_state, &self.duplicate_resource_record_response_parsing);
		let next_resource_record_pointer = self.loop_over_resource_records(next_resource_record_pointer, number_of_resource_records, AdditionalSectionError::ResourceRecordsOverflowSection, parse_method)?;
		
		self.response_parsing_state.parse_extended_dns_outcome::<Infallible>()?;
		
		Ok(next_resource_record_pointer)
	}
	
	#[inline(always)]
	fn loop_over_resource_records<E: error::Error>(&self, mut next_resource_record_pointer: usize, number_of_resource_records: u16, overflow_section_error: E, parse_method: impl for<'a> Fn(&ResourceRecord) -> Result<usize, E>) -> Result<usize, E>
	{
		for _ in 0 .. number_of_resource_records
		{
			if unlikely!(next_resource_record_pointer == self.end_of_message_pointer)
			{
				return Err(overflow_section_error)
			}
			let resource_record = next_resource_record_pointer.unsafe_cast::<ResourceRecord>();
			next_resource_record_pointer = parse_method(resource_record)?;
		}
		Ok(next_resource_record_pointer)
	}
}
