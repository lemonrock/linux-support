// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Query<'cache>
{
	message_identifier: MessageIdentifier,
	data_type: DataType,
	query_name: CaseFoldedName<'cache>,
}

impl<'cache> Query<'cache>
{
	#[inline(always)]
	pub fn enquire_over_tcp<'yielder, 'message, QP: QueryProcessor<'message, 'cache>, SD: SocketData>(stream: &mut TlsClientStream<'yielder, SD>, message_identifier: MessageIdentifier, query_name: CaseFoldedName<'cache>, cache: &mut Cache<'cache>) -> Result<(), ProtocolError<<QP as ResourceRecordVisitor<'message>>::Error>>
	where 'cache: 'message
	{
		let mut query = Query
		{
			message_identifier,
			data_type: QP::DT,
			query_name,
		};
		query.write_tcp_query(stream);
		query.read_tcp_reply(stream, cache)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	pub fn write_tcp_query<'yielder, SD: SocketData>(&self, stream: &mut TlsClientStream<'yielder, SD>)
	{
		let mut buffer: [u8; TcpDnsMessage::MaximumQueryBufferSize] = unsafe { uninitialized() };

		let buffer_pointer = (&mut buffer[..]).start_pointer();

		let buffer_length = TcpDnsMessage::write_query_tcp_message(buffer_pointer, self.message_identifier, self.data_type, &self.query_name);

		stream.write_all_data(&buffer[..])
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	pub fn read_tcp_reply<'yielder, 'message, QP: QueryProcessor<'message, 'cache>, SD: SocketData>(&self, stream: &mut TlsClientStream<'yielder, SD>, cache: &mut Cache<'cache>) -> Result<(), ProtocolError<QP::Error>>
	where 'cache: 'message
	{
		let mut buffer: [u8; ResourceRecord::UdpRequestorsPayloadSize] = unsafe { uninitialized() };
		let message_length = Self::reply_message(stream, &mut buffer)?;
		let raw_dns_message = &buffer[.. message_length];
		let mut answer_section_resource_record_visitor = QP::new();
		let (answer, canonical_name_chain_records, finished) = self.read_reply_after_message_length_checked(raw_dns_message, answer_section_resource_record_visitor).map_err(ProtocolError::ReadReplyAfterLengthChecked)?;
		QP::result(cache, answer, canonical_name_chain_records, finished)
	}
	
	#[inline(always)]
	fn read_reply_after_message_length_checked<'message, RRV: ResourceRecordVisitor<'message>>(&self, raw_dns_message: &'message [u8], answer_section_resource_record_visitor: RRV) -> Result<(Answer<'cache, CaseFoldedName<'cache>>, Records<'cache, CaseFoldedName<'cache>>, RRV::Finished), ReadReplyAfterLengthCheckedError<RRV::Error>>
	where 'cache: 'message
	{
		let now = NanosecondsSinceUnixEpoch::now();
		
		let dns_message = unsafe { &* (raw_dns_message.as_ptr() as *const DnsMessage) };
		let message_header = dns_message.message_header();
		let (authoritative_or_authenticated_or_neither, rcode_lower_4_bits) = self.parse_message_header(message_header)?;
		
		let start_of_message_pointer = raw_dns_message.start_pointer();
		let mut parsed_names = ParsedNames::new(start_of_message_pointer);
		
		let end_of_message_pointer = raw_dns_message.end_pointer();
		
		let (next_resource_record_pointer, query_name) = self.parse_query_section(dns_message.query_section_entry(), &mut parsed_names, end_of_message_pointer).map_err(SectionError::QuerySection)?;
		let response_record_sections_parser = ResponseRecordSectionsParser::new(now, self.data_type, end_of_message_pointer, message_header, parsed_names);
		let (end_of_parsed_message_pointer, answer, canonical_name_records, answer_section_resource_record_visitor_finished) = response_record_sections_parser.parse_answer_authority_and_additional_sections(next_resource_record_pointer, query_name, authoritative_or_authenticated_or_neither, rcode_lower_4_bits, answer_section_resource_record_visitor)?;
		
		if unlikely!(end_of_parsed_message_pointer < end_of_message_pointer)
		{
			return Err(ReadReplyAfterLengthCheckedError::MessageHadUnparsedBytesAtEnd(self.message_identifier))
		}
		
		Ok((answer, canonical_name_records, answer_section_resource_record_visitor_finished))
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn reply_message<'yielder, SD: SocketData>(stream: &mut TlsClientStream<'yielder, SD>, buffer: &mut [u8; ResourceRecord::UdpRequestorsPayloadSize]) -> Result<usize, MessageLengthError>
	{
		let message_length = Self::tcp_reply_message_length(stream)?;
		stream.read_all_data(&mut buffer[.. message_length]);
		Ok(message_length)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn tcp_reply_message_length<'yielder, SD: SocketData>(stream: &mut TlsClientStream<'yielder, SD>) -> Result<usize, MessageLengthError>
	{
		use self::MessageLengthError::*;
		
		let mut length_buffer: BigEndianU16 = unsafe { uninitialized() };
		stream.read_all_data(&mut length_buffer[..]);
		let message_length_big_endian: BigEndianU16 = (&length_buffer[0 .. TcpDnsMessage::TcpBufferLengthSize]).try_into().unwrap();
		let message_length = u16::from_be_bytes(message_length_big_endian) as usize;
		
		if unlikely!(message_length < DnsMessage::MinimumMessageSize)
		{
			Err(MessageIsTooShort(message_length))
		}
		else if unlikely!(message_length > ResourceRecord::UdpRequestorsPayloadSize)
		{
			Err(MessageIsTooLong(message_length))
		}
		else
		{
			Ok(message_length)
		}
	}
	
	#[inline(always)]
	fn parse_message_header(&self, message_header: &MessageHeader) -> Result<(AuthoritativeOrAuthenticatedOrNeither, RCodeLower4Bits), MessageHeaderError>
	{
		message_header.validate_is_not_query()?;
		message_header.validate_is_expected_reply(self.message_identifier)?;
		message_header.validate_contains_exactly_one_question()?;
		message_header.validate_response_message_opcode_is_query()?;
		message_header.validate_reserved_header_bits_are_zero()?;
		message_header.validate_response_is_not_truncated()?;
		message_header.validate_recursion_desired_bit_was_copied_from_query_and_is_one()?;
		message_header.validate_checking_bit_was_copied_from_query_and_is_zero()?;
		
		let authoritative_or_authenticated_or_neither = message_header.validate_authoritative_or_authenticated()?;
		let rcode_lower_4_bits = message_header.raw_message_response_code();
		Ok((authoritative_or_authenticated_or_neither, rcode_lower_4_bits))
	}
	
	#[inline(always)]
	fn parse_query_section<'message>(&self, query_section_entry: &'message QuerySectionEntry, parsed_names: &mut ParsedNames, end_of_message_pointer: usize) -> Result<(usize, ParsedName<'message>), QuerySectionError>
	where 'cache: 'message
	{
		use self::QuerySectionError::*;
		
		let (parsed_query_name, end_of_qname_pointer) = ParsedNameParser::parse_name(Some(ParsedNameParserError::CompressedNameLabelsAreDisallowedInQuerySection), parsed_names, query_section_entry.start_of_name_pointer(), end_of_message_pointer)?;
		
		query_section_entry.validate_is_internet_query_class(end_of_qname_pointer)?;
		
		if unlikely!(self.data_type != query_section_entry.data_type(end_of_qname_pointer))
		{
			return Err(ResponseWasForADifferentDataType)
		}
		
		if unlikely!(self.query_name.name() != parsed_query_name)
		{
			Err(ResponseWasForADifferentName)
		}
		
		let end_of_query_section = QuerySectionEntry::end_of_query_section(end_of_qname_pointer);
		Ok((end_of_query_section, parsed_query_name))
	}
	
}
