// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Name consists of one or more labels.
pub(crate) struct Name;

impl Name
{
	/// The smallest Name consists of one label, which is the Root label, which is one byte.
	pub(crate) const MinimumSize: usize = 1;

	pub(crate) const MaximumSize: usize = 255;

	#[inline(always)]
	pub(crate) fn parse_without_compression_but_register_labels_for_compression<'message>(&'message mut self, parsed_labels: &mut ParsedLabels, end_of_message_pointer: usize) -> Result<(WithoutCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		parsed_labels.parse_without_compression_but_register_labels_for_compression(self.as_usize_pointer_mut(), end_of_message_pointer)
	}

	#[inline(always)]
	pub(crate) fn parse_with_compression<'message>(&'message mut self, parsed_labels: &mut ParsedLabels, end_of_message_pointer: usize) -> Result<(WithCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		parsed_labels.parse_name(self.as_usize_pointer_mut(), end_of_message_pointer)
	}

	#[inline(always)]
	pub(crate) fn maximum_for_end_of_name_pointer(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		let maximum_potential_name_length = Self::maximum_potential_name_length(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		let end_of_name_data_pointer = start_of_name_pointer + maximum_potential_name_length;
		Ok(end_of_name_data_pointer)
	}

	#[inline(always)]
	fn maximum_potential_name_length(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer occurs before start_of_name_pointer");

		if unlikely!(start_of_name_pointer == end_of_data_section_containing_name_pointer)
		{
			return Err(NameIsEmpty)
		}

		let unconstrained_maximum_potential_name_length = end_of_data_section_containing_name_pointer - start_of_name_pointer;
		Ok(min(unconstrained_maximum_potential_name_length, Self::MaximumSize))
	}
}
