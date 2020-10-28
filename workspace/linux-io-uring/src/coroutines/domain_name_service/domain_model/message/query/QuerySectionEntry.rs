// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct QuerySectionEntry;

impl QuerySectionEntry
{
	const MaximumQueryNameSize: usize = ParsedNameParser::NameMaximumSize;
	
	const QueryTypeSize: usize = 2;
	
	const ClassSize: usize = 2;
	
	pub(crate) const MaximumSizeOfOneQuery: usize = ParsedNameParser::NameMaximumSize + Self::QueryTypeSize + Self::ClassSize;

	/// Validation of available buffer size is done before calling this.
	#[inline(always)]
	pub(crate) fn write_query_section_entry_for_query(query_section_pointer: usize, data_type: DataType, query_name: &CaseFoldedName<'_>) -> usize
	{
		let mut current_pointer = query_name.copy_non_overlapping_to_without_case_folding(query_section_pointer);

		current_pointer.set_u16_bytes(data_type.0);
		current_pointer += Self::QueryTypeSize;

		current_pointer.set_u16_bytes([0x00, QueryClass::Internet as u8]);
		current_pointer + Self::ClassSize
	}
	
	#[inline(always)]
	pub(crate) fn data_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.query_type_or_data_type(end_of_name_pointer).data_type()
	}

	#[inline(always)]
	pub(crate) const fn end_of_query_section(end_of_qname_pointer: usize) -> usize
	{
		const QueryTypeOrDataTypeSize: usize = 2;
		const QueryClassSize: usize = 2;
		end_of_qname_pointer + QueryTypeOrDataTypeSize + QueryClassSize
	}

	/// `QNAME` field.
	#[inline(always)]
	pub(crate) fn start_of_name_pointer(&self) -> usize
	{
		self.as_usize_pointer()
	}

	/// `QTYPE` field.
	#[inline(always)]
	pub(crate) fn query_type_or_data_type(&self, end_of_name_pointer: usize) -> QueryTypeOrDataType
	{
		self.query_section_entry_footer(end_of_name_pointer).query_type_or_data_type()
	}

	/// `QCLASS` field.
	#[inline(always)]
	pub(crate) fn validate_is_internet_query_class(&self, end_of_name_pointer: usize) -> Result<(), QuerySectionError>
	{
		self.query_section_entry_footer(end_of_name_pointer).validate_is_internet_query_class()
	}

	#[inline(always)]
	fn query_section_entry_footer(&self, end_of_name_pointer: usize) -> &QuerySectionEntryFooter
	{
		end_of_name_pointer.unsafe_cast::<QuerySectionEntryFooter>()
	}
}
