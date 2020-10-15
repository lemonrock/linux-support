// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C, packed)]
pub(crate) struct TcpMessage
{
	length: [u8; TcpMessage::TcpBufferLengthSize],
	message: Message,
}

impl TcpMessage
{
	pub(crate) const TcpBufferLengthSize: usize = size_of::<u16>();

	pub(crate) const MaximumQueryBufferSize: usize = Self::TcpBufferLengthSize + Message::MaximumQueryMessageSize;

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	pub(crate) fn write_query_tcp_message(buffer_pointer: usize, message_identifier: MessageIdentifier, data_type: DataType, query_name: &UncompressedName<impl Allocator>) -> usize
	{
		let message_pointer = buffer_pointer + TcpMessage::TcpBufferLengthSize;

		let query_section_pointer = MessageHeader::write_message_header(message_pointer, message_identifier);
		let query_section_end_pointer = QuerySectionEntry::write_query_section_entry_for_query(query_section_pointer, data_type, query_name);
		let end_pointer = ResourceRecord::write_extended_dns_0_opt_for_query(query_section_end_pointer);
		TcpMessage::write_tcp_buffer_length(buffer_pointer, message_pointer, end_pointer);

		end_pointer - buffer_pointer
	}

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	fn write_tcp_buffer_length(buffer_pointer: usize, message_pointer: usize, end_pointer: usize)
	{
		buffer_pointer.set_u16_network_endian_from_usize(end_pointer - message_pointer)
	}

	/// Length (excluding the two byte length field).
	#[inline(always)]
	pub fn length(&self) -> u16
	{
		self.length.from_network_endian_to_native_endian()
	}

	/// Message.
	#[inline(always)]
	pub fn message(&self) -> &Message
	{
		&self.message
	}
}
