// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DNS message header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, packed)]
pub(crate) struct MessageHeader
{
	identifier: MessageIdentifier,
	bitfield1: MessageBitField1,
	bitfield2: MessageBitField2,
	qd_count: BigEndianU16,
	an_count: BigEndianU16,
	ns_count: BigEndianU16,
	ar_count: BigEndianU16,
}

impl MessageHeader
{
	const MessageIdentifierSize: usize = size_of::<MessageIdentifier>();

	const Bitfield1Size: usize = size_of::<MessageBitField1>();

	const Bitfield2Size: usize = size_of::<MessageBitField2>();

	const QueryCountSize: usize = size_of::<BigEndianU16>();

	const AnswerCountSize: usize = size_of::<BigEndianU16>();

	const AuthorityCountSize: usize = size_of::<BigEndianU16>();

	const AdditionalCountSize: usize = size_of::<BigEndianU16>();

	pub(crate) const Size: usize = size_of::<Self>();

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	pub(crate) fn write_query_message_header(message_pointer: usize, message_identifier: MessageIdentifier) -> usize
	{
		let mut current_pointer = message_pointer;
		
		current_pointer.set_u16_bytes(message_identifier.0);
		current_pointer += Self::MessageIdentifierSize;
		
		current_pointer.set_u8_byte(MessageBitField1::new_for_query());
		current_pointer += Self::Bitfield1Size;
		
		current_pointer.set_u8_byte(MessageBitField2::new_for_query());
		current_pointer += Self::Bitfield2Size;
	
		const QuestionCount: u16 = 1;
		current_pointer.set_u16_bytes(QuestionCount.to_be_bytes());
		current_pointer += Self::QueryCountSize;
		
		const AnswerCount: u16 = 0;
		current_pointer.set_u16_bytes(AnswerCount.to_be_bytes());
		current_pointer += Self::QueryCountSize;
		
		const AuthorityCount: u16 = 0;
		current_pointer.set_u16_bytes(AuthorityCount.to_be_bytes());
		current_pointer += Self::QueryCountSize;
		
		const AdditionalCount: u16 = 1; // For EDNS(0) `OPT` record.
		current_pointer.set_u16_bytes(AdditionalCount.to_be_bytes());
		current_pointer += Self::AdditionalCount;
		
		current_pointer
	}
	
	#[inline(always)]
	pub(crate) fn is_query(&self) -> bool
	{
		self.query_response() == MessageType::Query
	}

	/// `ID` field.
	#[inline(always)]
	pub(crate) fn identifier(&self) -> MessageIdentifier
	{
		self.identifier
	}

	/// `QR` field.
	///
	/// A one bit field that specifies whether this message is a query or a response.
	#[inline(always)]
	pub(crate) fn query_response(&self) -> MessageType
	{
		self.bitfield1.query_response()
	}

	/// `Opcode` field.
	///
	/// A four bit field that specifies kind of query in this message.
	///
	/// This value is set by the originator of a query and copied into the response.
	///
	/// Only values 0 - 6 inclusive are defined by IANA, and some of those are for drafts, reserved or obsolete.
	///
	/// Valid codes are of type `MessageOpcode`.
	#[inline(always)]
	pub(crate) fn raw_message_opcode(&self) -> u8
	{
		self.bitfield1.raw_message_opcode()
	}

	/// `AA` field.
	///
	/// This bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section.
	///
	/// Note that the contents of the answer section may have multiple owner names because of aliases.
	///
	/// The `AA` field corresponds to the name which matches the query name, or the first owner name in the answer section.
	#[inline(always)]
	pub(crate) fn authoritative_answer(&self) -> bool
	{
		self.bitfield1.authoritative_answer()
	}

	/// `TC` field.
	///
	/// Is this message truncated due to limitations on packet sizes on the underlying transport?
	#[inline(always)]
	pub(crate) fn is_truncated(&self) -> bool
	{
		self.bitfield1.is_truncated()
	}

	/// `RD` field.
	///
	/// This bit may be set in a query and is copied into the response.
	///
	/// If `RD` is set, it directs the name server to pursue the query recursively.
	///
	/// Recursive query support is optional.
	#[inline(always)]
	pub fn recursion_desired(&self) -> bool
	{
		self.bitfield1.recursion_desired()
	}

	/// `RA` field.
	///
	/// This bit is set or cleared in a response, and denotes whether recursive query support is available in the name server.
	#[inline(always)]
	pub(crate) fn recursion_available(&self) -> bool
	{
		self.bitfield2.recursion_available()
	}

	/// `Z` field.
	///
	/// Reserved for future use.
	///
	/// Must be zero in all queries and responses.
	///
	/// Some ancient DNS clients set this to `1` to tell a DNS server that only a response from the primary DNS server for a zone is acceptable.
	#[inline(always)]
	pub(crate) fn z(&self) -> bool
	{
		self.bitfield2.z()
	}

	/// `AD` field.
	///
	/// Defined in RFC 2535.
	///
    /// From [RFC 4035, DNSSEC Resource Records, March 2005](https://tools.ietf.org/html/rfc4035#section-3.1.6):-
    ///
    /// ```text
    ///
    /// 3.1.6.  The AD and CD Bits in an Authoritative Response
    ///
    ///   The CD and AD bits are designed for use in communication between
    ///   security-aware resolvers and security-aware recursive name servers.
    ///   These bits are for the most part not relevant to query processing by
    ///   security-aware authoritative name servers.
    ///
    ///   A security-aware name server does not perform signature validation
    ///   for authoritative data during query processing, even when the CD bit
    ///   is clear.  A security-aware name server SHOULD clear the CD bit when
    ///   composing an authoritative response.
    ///
    ///   A security-aware name server MUST NOT set the AD bit in a response
    ///   unless the name server considers all RRsets in the Answer and
    ///   Authority sections of the response to be authentic.  A security-aware
    ///   name server's local policy MAY consider data from an authoritative
    ///   zone to be authentic without further validation.  However, the name
    ///   server MUST NOT do so unless the name server obtained the
    ///   authoritative zone via secure means (such as a secure zone transfer
    ///   mechanism) and MUST NOT do so unless this behavior has been
    ///   configured explicitly.
    ///
    ///   A security-aware name server that supports recursion MUST follow the
    ///   rules for the CD and AD bits given in Section 3.2 when generating a
    ///   response that involves data obtained via recursion.
	/// ```
	#[inline(always)]
	pub(crate) fn authentic_data(&self) -> bool
	{
		self.bitfield2.authentic_data()
	}

	/// `CD` field.
	///
	/// Defined in RFC 2535.
	///
	/// See documentation for `authentic_data()`.
	#[inline(always)]
	pub(crate) fn checking_disabled(&self) -> bool
	{
		self.bitfield2.checking_disabled()
	}

	/// `RCODE` field.
	///
	/// A four bit field that specifies the response outcome.
	///
	/// Valid codes are of type `MessageResponseCode`.
	#[inline(always)]
	pub(crate) fn raw_message_response_code(&self) -> u8
	{
		self.bitfield2.raw_message_response_code()
	}

	/// `QDCOUNT` field.
	///
	/// The number of entries in the question section.
	///
	/// Only a value of 1 is normally encountered.
	#[inline(always)]
	pub(crate) fn number_of_entries_in_the_question_section(&self) -> u16
	{
		self.qd_count.from_network_endian_to_native_endian()
	}

	/// `ANCOUNT` field.
	///
	/// The number of resource records in the answer section.
	#[inline(always)]
	pub(crate) fn number_of_resource_records_in_the_answer_section(&self) -> u16
	{
		self.an_count.from_network_endian_to_native_endian()
	}

	/// `NSCOUNT` field.
	///
	/// The number of name server resource records in the authority records section.
	#[inline(always)]
	pub(crate) fn number_of_resource_records_in_the_authority_records_section(&self) -> u16
	{
		self.ns_count.from_network_endian_to_native_endian()
	}

	/// `ARCOUNT` field.
	///
	/// The number of resource records in the additional records section.
	#[inline(always)]
	pub(crate) fn number_of_resource_records_in_the_additional_records_section(&self) -> u16
	{
		self.ar_count.from_network_endian_to_native_endian()
	}
}
