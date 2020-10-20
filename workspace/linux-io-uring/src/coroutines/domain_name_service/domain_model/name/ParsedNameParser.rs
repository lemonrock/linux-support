// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) struct ParsedNameParser<'a>
{
	permit_compressed_names: bool,
	parsed_names: &'a mut ParsedNames,
	start_of_name_pointer: usize,
	maximum_for_end_of_name_pointer: usize,
	label_data_starts_at_pointers_and_label_length_excluding_trailing_period: ArrayVec<[(usize, u8); Label::MaximumNumber]>,
	name_length_including_trailing_periods_after_labels: u8,
}

impl<'a> ParsedNameParser<'a>
{
	const SizeOfTrailingPeriod: u8 = 1;
	
	/// The smallest Name consists of one label, which is the Root label, which is one byte.
	pub(crate) const NameMinimumSize: usize = 1;
	
	pub(crate) const NameMaximumSize: usize = 255;
	
	#[inline(always)]
	pub(crate) fn new(permit_compressed_names: bool, parsed_names: &'a mut ParsedNames, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<Self, DnsProtocolError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer {} occurs before start_of_name_pointer {}", end_of_data_section_containing_name_pointer, start_of_name_pointer);
		
		Ok
		(
			Self
			{
				permit_compressed_names,
				parsed_names,
				start_of_name_pointer,
				maximum_for_end_of_name_pointer: Self::maximum_for_end_of_name_pointer(start_of_name_pointer, end_of_data_section_containing_name_pointer)?,
				label_data_starts_at_pointers_and_label_length_excluding_trailing_period: ArrayVec::new(),
				name_length_including_trailing_periods_after_labels: 0,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn parse_name<'message>(mut self) -> Result<(ParsedName<'message>, usize), DnsProtocolError>
	{
		let mut label_starts_at_pointer = self.start_of_name_pointer;
		let end_of_qname_pointer = loop
		{
			match self.parsed_label(label_starts_at_pointer)
			{
				Left(next_label_starts_at_pointer) =>
				{
					label_starts_at_pointer = next_label_starts_at_pointer;
				}
				
				Right(end_of_qname_pointer) => break end_of_qname_pointer,
			}
		};
		
		self.parsed_names.register(&self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period);
		
		Ok
		(
			(
				ParsedName::new(self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period, self.name_length_including_trailing_periods_after_labels),
				end_of_qname_pointer
			)
		)
	}
	
	#[inline(always)]
	fn parse_label(&mut self, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, DnsProtocolError>
	{
		use self::LabelKind::*;
		
		if unlikely!(label_starts_at_pointer == self.maximum_for_end_of_name_pointer)
		{
			return Err(NoTerminalRootLabel)
		}
		
		let raw_label = RawLabel::raw_label(label_starts_at_pointer);
		
		match raw_label.label_kind()
		{
			Bytes => self.parse_bytes_label_kind(raw_label, label_starts_at_pointer),
			
			CompressedOffsetPointer => self.parse_compressed_label_kind(raw_label, label_starts_at_pointer),
			
			Extended => Err(ExtendedNameLabelsAreUnused),
			
			Unallocated => Err(UnallocatedNameLabelsAreUnused),
		}
	}
	
	#[inline(always)]
	fn parse_bytes_label_kind(&mut self, raw_label: &RawLabel, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, DnsProtocolError>
	{
		let label_length_excluding_trailing_period = raw_label.bytes_length();
		let label_length_including_trailing_period = label_length_excluding_trailing_period + Self::SizeOfTrailingPeriod;
		self.name_length_including_trailing_periods_after_labels.checked_add(label_length_including_trailing_period).ok_or(LabelPointerCreatesADnsNameLongerThan255Bytes)?;
		
		let (label_data_starts_at_pointer, next_label_starts_at_pointer) = self.validate_label_length_does_not_cause_overflow(label_starts_at_pointer, label_length_excluding_trailing_period)?;
		self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.try_push((label_data_starts_at_pointer, label_length_excluding_trailing_period)).map_err(|| LabelPointerCreatesADnsNameLongerThan127Labels)?;
		
		let is_terminal_root_label = label_length_excluding_trailing_period == 0;
		if unlikely!(is_terminal_root_label)
		{
			Ok(Right(next_label_starts_at_pointer))
		}
		else
		{
			Ok(Left(next_label_starts_at_pointer))
		}
	}
	
	#[inline(always)]
	fn parse_compressed_label_kind(&mut self, raw_label: &RawLabel, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, DnsProtocolError>
	{
		if self.permit_compressed_names
		{
			let compressed_pointer_offset = raw_label.compressed_pointer_offset();
			let parsed_name = self.parsed_names.look_up(compressed_pointer_offset)?;
			let name_length_including_trailing_periods_after_labels = parsed_name.name_length_including_trailing_periods_after_labels.get();
			let label_data_starts_at_pointers_and_label_length_excluding_trailing_period = &parsed_name.label_data_starts_at_pointers_and_label_length_excluding_trailing_period[..];
			
			self.name_length_including_trailing_periods_after_labels.checked_add(name_length_including_trailing_periods_after_labels).ok_or(CompressedLabelPointerCreatesADnsNameLongerThan255Bytes)?;
			
			self.label_data_starts_at_pointers_and_label_length_excluding_trailing_period.try_extend_from_slice(label_data_starts_at_pointers_and_label_length_excluding_trailing_period).map_err(|| CompressedLabelPointerCreatesADnsNameLongerThan127Labels)?;
			
			let next_label_starts_at_pointer = label_starts_at_pointer + LabelKind::CompressedOffsetPointerLabelKindSize;
			if likely!(next_label_starts_at_pointer <= self.maximum_for_end_of_name_pointer)
			{
				Ok(Right(label_starts_at_pointer + LabelKind::CompressedOffsetPointerLabelKindSize))
			}
			else
			{
				Err(LabelPointerOverflows)
			}
		}
		else
		{
			Err(CompressedNameLabelsAreDisallowedInThisResourceRecord)
		}
	}
	
	#[inline(always)]
	fn validate_label_length_does_not_cause_overflow(&self, label_starts_at_pointer: usize, label_length_excluding_trailing_period: u8) -> Result<(usize, usize), DnsProtocolError>
	{
		let label_data_starts_at = label_starts_at_pointer + LabelKind::BytesLabelKindSize;
		let next_label_starts_at_pointer = label_data_starts_at + (label_length_excluding_trailing_period as usize);
		if likely!(next_label_starts_at_pointer <= self.maximum_for_end_of_name_pointer)
		{
			Ok((label_data_starts_at, next_label_starts_at_pointer))
		}
		else
		{
			Err(LabelLengthOverflows)
		}
	}

	#[inline(always)]
	fn maximum_for_end_of_name_pointer(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		let maximum_potential_name_length = Self::maximum_potential_name_length(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		let maximum_for_end_of_name_pointer = start_of_name_pointer + maximum_potential_name_length;
		Ok(maximum_for_end_of_name_pointer)
	}

	#[inline(always)]
	fn maximum_potential_name_length(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, DnsProtocolError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer {} occurs before start_of_name_pointer {}", end_of_data_section_containing_name_pointer, start_of_name_pointer);

		if unlikely!(start_of_name_pointer == end_of_data_section_containing_name_pointer)
		{
			return Err(NameIsEmpty)
		}

		let unconstrained_maximum_potential_name_length = end_of_data_section_containing_name_pointer - start_of_name_pointer;
		Ok(min(unconstrained_maximum_potential_name_length, ParsedNameParser::NameMaximumSize))
	}
}
