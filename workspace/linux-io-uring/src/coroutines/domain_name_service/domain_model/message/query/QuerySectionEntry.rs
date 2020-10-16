// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct QuerySectionEntry;

impl QuerySectionEntry
{
	const MaximumQueryNameSize: usize = Name::MaximumSize;
	
	const QueryTypeSize: usize = 2;
	
	const ClassSize: usize = 2;
	
	const MaximumSizeOfOneQuery: usize = Name::MaximumSize + Self::QueryTypeSize + Self::ClassSize;

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	pub(crate) fn write_query_section_entry_for_query(query_section_pointer: usize, data_type: DataType, query_name: &UncompressedName<impl Allocator>) -> usize
	{
		let mut current_pointer = query_name.copy_non_overlapping_to(query_section_pointer);

		current_pointer.set_u16_bytes(data_type.0);
		current_pointer += Self::QueryTypeSize;

		current_pointer.set_u16_bytes([0x00, QueryClass::Internet as u8]);
		current_pointer + Self::ClassSize
	}

	#[inline(always)]
	pub(crate) fn parse_response<'message>(&'message mut self, parsed_labels: &mut ParsedLabels, end_of_message_pointer: usize, request_query_identification: Query<impl Allocator>) -> Result<(usize, DataType), DnsProtocolError>
	{
		let (qname, end_of_qname_pointer) = self.name().parse_without_compression_but_register_labels_for_compression(parsed_labels, end_of_message_pointer)?;

		let query_class = self.query_class(end_of_qname_pointer)?;
		debug_assert_eq!(query_class, QueryClass::Internet);

		let data_type = self.data_type(end_of_qname_pointer);
		request_query_identification.matches(data_type, qname)?;

		Ok((Self::end_of_query_section(end_of_qname_pointer), data_type))
	}

	#[inline(always)]
	fn data_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.query_type_or_data_type(end_of_name_pointer).data_type()
	}

	#[inline(always)]
	const fn end_of_query_section(end_of_qname_pointer: usize) -> usize
	{
		const QueryTypeOrDataTypeSize: usize = 2;
		const QueryClassSize: usize = 2;
		end_of_qname_pointer + QueryTypeOrDataTypeSize + QueryClassSize
	}

	/// `QNAME` field.
	#[inline(always)]
	pub(crate) fn name(&mut self) -> &mut Name
	{
		self.unsafe_cast_mut::<Name>()
	}

	/// `QTYPE` field.
	#[inline(always)]
	pub(crate) fn query_type_or_data_type(&self, end_of_name_pointer: usize) -> QueryTypeOrDataType
	{
		self.query_section_entry_footer(end_of_name_pointer).query_type_or_data_type()
	}

	/// `QCLASS` field.
	#[inline(always)]
	pub(crate) fn query_class(&self, end_of_name_pointer: usize) -> Result<QueryClass, DnsProtocolError>
	{
		self.query_section_entry_footer(end_of_name_pointer).query_class()
	}

	#[inline(always)]
	fn query_section_entry_footer(&self, end_of_name_pointer: usize) -> &QuerySectionEntryFooter
	{
		end_of_name_pointer.unsafe_cast::<QuerySectionEntryFooter>()
	}
}
