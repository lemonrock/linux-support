// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedLabels
{
	pub(crate) start_of_message_pointer: usize,

	parsed_labels: std::collections::HashMap<u16, ParsedLabelInformation>,
}

impl ParsedLabels
{
	#[inline(always)]
	pub(crate) fn new(start_of_message_pointer: usize) -> Self
	{
		Self
		{
			start_of_message_pointer,
			parsed_labels: HashMap::with_capacity(128),
		}
	}

	#[inline(always)]
	pub(crate) fn parse_name_in_slice_with_nothing_left<'message>(&mut self, slice: &'message mut [u8]) -> Result<WithCompressionParsedName<'message>, DnsProtocolError>
	{
		match self.parse_name_in_slice(slice)
		{
			Err(error) => Err(error),

			Ok((parsed_name_iterator, end_of_name_pointer)) => if unlikely!(end_of_name_pointer - slice.len() != slice.as_ptr() as usize)
			{
				Err(NameWasNotLongEnough)
			}
			else
			{
				Ok(parsed_name_iterator)
			}
		}
	}

	#[inline(always)]
	pub(crate) fn parse_name_in_slice<'message>(&mut self, slice: &'message mut [u8]) -> Result<(WithCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		let length = slice.len();
		if unlikely!(length == 0)
		{
			return Err(NameIsEmpty)
		}

		let start_of_name_pointer = slice.as_ptr() as usize;
		self.parse_name(start_of_name_pointer, start_of_name_pointer + length)
	}

	#[inline(always)]
	pub(crate) fn parse_without_compression_but_register_labels_for_compression<'message>(&mut self, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(WithoutCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		WithoutCompressionParsedName::parse_without_compression_but_register_labels_for_compression(self, start_of_name_pointer, end_of_data_section_containing_name_pointer)
	}

	#[inline(always)]
	pub(crate) fn parse_name<'message>(&mut self, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(WithCompressionParsedName<'message>, usize), DnsProtocolError>
	{
		WithCompressionParsedName::parse_with_compression(self, start_of_name_pointer, end_of_data_section_containing_name_pointer)
	}

	#[inline(always)]
	pub(crate) fn guard(&self, offset: usize, start_of_name_pointer: usize, labels_register_reference: &mut LabelsRegister) -> Result<(usize, u8, u8), DnsProtocolError>
	{
		debug_assert!(offset <= ::std::u16::MAX as usize, "offset is larger than ::std::u16::MAX");

		let points_to_label_at = self.start_of_message_pointer + offset;

		let pointer_points_at_or_after_start_of_name = points_to_label_at >= start_of_name_pointer;
		if unlikely!(pointer_points_at_or_after_start_of_name)
		{
			return Err(LabelPointerPointsToDataAfterTheStartOfTheCurrentlyBeingParsedName)
		}

		let compressed_offset = points_to_label_at as u16;
		let &ParsedLabelInformation { mut number_of_uncompressed_labels_with_all_pointers_resolved, mut length_of_all_labels_including_period } = self.parsed_labels.get(&compressed_offset).ok_or(LabelPointerPointsToALabelThatWasNotPreviouslyParsed(offset))?;

		let number_of_labels = number_of_uncompressed_labels_with_all_pointers_resolved + labels_register_reference.len() as u8;
		if unlikely!(number_of_labels > Label::MaximumNumber as u8)
		{
			return Err(LabelPointerCreatesADnsNameLongerThan127Labels)
		}

		for (label_starts_at_pointer, label_bytes_length_including_trailing_period) in labels_register_reference.iter().rev()
		{
			number_of_uncompressed_labels_with_all_pointers_resolved += 1;

			let label_starts_at_pointer = *label_starts_at_pointer;
			let label_bytes_length_including_trailing_period = *label_bytes_length_including_trailing_period;

			debug_assert_ne!(label_bytes_length_including_trailing_period, 0, "label_bytes_length_including_trailing_period was zero");
			length_of_all_labels_including_period = length_of_all_labels_including_period.checked_add(label_bytes_length_including_trailing_period).ok_or(LabelPointerCreatesADnsNameLongerThan255Bytes)?;

			debug_assert!(label_starts_at_pointer >= self.start_of_message_pointer, "offset occurs before start_of_message_pointer");
			let offset = label_starts_at_pointer - self.start_of_message_pointer;
			debug_assert!(offset <= ::std::u16::MAX as usize, "offset `{}` exceeds ::std::u16::MAX", offset);
			let previous = self.parsed_labels.insert(offset as u16, ParsedLabelInformation { number_of_uncompressed_labels_with_all_pointers_resolved, length_of_all_labels_including_period });
			debug_assert_eq!(previous, None, "duplicate uncompressed label");
		}
		Ok((points_to_label_at, number_of_labels, length_of_all_labels_including_period))
	}
}
