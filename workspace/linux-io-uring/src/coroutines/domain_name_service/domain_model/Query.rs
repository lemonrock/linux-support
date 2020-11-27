// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Query<'query_name>
{
	now: NanosecondsSinceUnixEpoch,
	message_identifier: MessageIdentifier,
	data_type: DataType,
	query_name: &'query_name FullyQualifiedDomainName,
}

impl<'query_name> Query<'query_name>
{
	#[inline(always)]
	pub(crate) fn enquire_over_tcp<'yielder, SD: SocketData, QP: QueryProcessor>(stream: &mut TlsClientStream<'yielder, SD>, message_identifier: MessageIdentifier, now: NanosecondsSinceUnixEpoch, query_name: &'query_name FullyQualifiedDomainName, domain_cache: &mut DomainCache) -> Result<(), ProtocolError<Infallible>>
	{
		let query = Query
		{
			now,
			message_identifier,
			data_type: QP::DT,
			query_name,
		};
		query.write_tcp_query(stream);
		query.read_tcp_reply::<QP, SD>(stream, domain_cache)
	}
	
	#[inline(always)]
	pub(crate) fn write_tcp_query<'yielder, SD: SocketData>(&self, stream: &mut TlsClientStream<'yielder, SD>)
	{
		let mut buffer: [u8; TcpDnsMessage::MaximumQueryBufferSize] = unsafe_uninitialized();

		let buffer_pointer = (&mut buffer[..]).start_pointer();

		let buffer_length = TcpDnsMessage::write_query_tcp_message(buffer_pointer, self.message_identifier, self.data_type, self.query_name);

		stream.write_all_data(&buffer[.. buffer_length])
	}
	
	#[inline(always)]
	pub(crate) fn read_tcp_reply<'yielder, SD: SocketData, QP: QueryProcessor>(&self, stream: &mut TlsClientStream<'yielder, SD>, domain_cache: &mut DomainCache) -> Result<(), ProtocolError<Infallible>>
	{
		let mut buffer: [u8; ResourceRecord::UdpRequestorsPayloadSize] = unsafe_uninitialized();
		let message_length = Self::reply_message(stream, &mut buffer)?;
		let raw_dns_message = &buffer[.. message_length];
		
		
		let answer_section_resource_record_visitor = QP::new(self.query_name);
		let (answer, canonical_name_chain) = self.read_reply_after_message_length_checked(raw_dns_message, answer_section_resource_record_visitor).map_err(ProtocolError::ReadReplyAfterLengthChecked)?;
		domain_cache.answered(answer, canonical_name_chain)?;
		Ok(())
	}
	
	#[inline(always)]
	fn read_reply_after_message_length_checked<'message, RRV: ResourceRecordVisitor<'message, Finished=OwnerNameToParsedRecords<'message, PR>>, PR: ParsedRecord>(&self, raw_dns_message: &'message [u8], answer_section_resource_record_visitor: RRV) -> Result<(Answer<PR>, CanonicalNameChain<'message>), ReadReplyAfterLengthCheckedError<RRV::Error>>
	{
		let dns_message = unsafe { &* (raw_dns_message.as_ptr() as *const DnsMessage) };
		let message_header = dns_message.message_header();
		let (authoritative_or_authenticated_or_neither, rcode_lower_4_bits) = self.parse_message_header(message_header)?;
		
		let start_of_message_pointer = raw_dns_message.start_pointer();
		let mut parsed_names = ParsedNames::new(start_of_message_pointer);
		
		let end_of_message_pointer = raw_dns_message.end_pointer();
		
		let next_resource_record_pointer = self.parse_query_section(dns_message.query_section_entry(), &mut parsed_names, end_of_message_pointer).map_err(SectionError::QuerySection)?;
		let mut response_record_sections_parser = ResponseRecordSectionsParser::new(self.now, self.data_type, end_of_message_pointer, message_header, parsed_names, self.query_name);
		let (end_of_parsed_message_pointer, answer, canonical_name_chain) = response_record_sections_parser.parse_answer_authority_and_additional_sections(next_resource_record_pointer, authoritative_or_authenticated_or_neither, rcode_lower_4_bits, answer_section_resource_record_visitor)?;
		
		if unlikely!(end_of_parsed_message_pointer < end_of_message_pointer)
		{
			return Err(ReadReplyAfterLengthCheckedError::MessageHadUnparsedBytesAtEnd(self.message_identifier))
		}
		
		Ok((answer, canonical_name_chain))
	}
	
	#[inline(always)]
	fn reply_message<'yielder, SD: SocketData>(stream: &mut TlsClientStream<'yielder, SD>, buffer: &mut [u8; ResourceRecord::UdpRequestorsPayloadSize]) -> Result<usize, MessageLengthError>
	{
		let message_length = Self::tcp_reply_message_length(stream)?;
		stream.read_all_data(&mut buffer[.. message_length]);
		Ok(message_length)
	}
	
	#[inline(always)]
	fn tcp_reply_message_length<'yielder, SD: SocketData>(stream: &mut TlsClientStream<'yielder, SD>) -> Result<usize, MessageLengthError>
	{
		use self::MessageLengthError::*;
		
		let mut length_buffer: BigEndianU16 = unsafe_uninitialized();
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
	fn parse_query_section<'message>(&self, query_section_entry: &'message QuerySectionEntry, parsed_names: &mut ParsedNames<'message>, end_of_message_pointer: usize) -> Result<usize, QuerySectionError>
	{
		use self::QuerySectionError::*;
		
		let (parsed_query_name, end_of_qname_pointer) = ParsedNameParser::parse_name(Some(ParsedNameParserError::CompressedNameLabelsAreDisallowedInQuerySection), parsed_names, query_section_entry.start_of_name_pointer(), end_of_message_pointer)?;
		
		query_section_entry.validate_is_internet_query_class(end_of_qname_pointer)?;
		
		if unlikely!(self.data_type != query_section_entry.data_type(end_of_qname_pointer))
		{
			return Err(ResponseWasForADifferentDataType)
		}
		
		if unlikely!(self.query_name.ne(&parsed_query_name))
		{
			return Err(ResponseWasForADifferentName)
		}
		
		let end_of_query_section = QuerySectionEntry::end_of_query_section(end_of_qname_pointer);
		Ok(end_of_query_section)
	}
}
