// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Compression of names within `RDATA` is a mess.
///
/// RFC 3597, Section 4, Paragraph 2 restricts the records to which name (label) compression can be applied to be those defined in RFC 1035 which implicitly contain a name, hence:-
///
/// * `CNAME`
/// * `MB`
/// * `MD`
/// * `MF`
/// * `MG`
/// * `MINFO`
/// * `MR`
/// * `MX`
/// * `NS`
/// * `PTR`
/// * `SOA`
///
/// Of these, many are obsolete, leaving the list as:-
///
/// * `CNAME`
/// * `MX`
/// * `NS`
/// * `PTR`
/// * `SOA`
///
/// Additionally:-
///
/// * RFC 2163 permits compression to `PX` records;
/// * RFC 2535 permits compression in `SIG` and `NXT` records;
/// * RFC 3597 permits compression in `RP`, `AFSDB`, `RT` and `NAPTR` records;
/// * RFC 3597 prohibits compression in `PX`, `SIG` and `NXT` records;
/// * RFC 2782 prohibits compression in `SRV` records but the original RFC 2052 mandated it;
/// * RFC 3597 prohibits compression for all future record types;
/// * RFC 6672 prohibits compression for `DNAME`, but historically, there was confusion in the original RFC 2672 about whether it was permitted.
///
/// Of the records listed in the clause above, all are obsolete apart from `NAPTR`, `SRV` and `DNAME`.
///
/// Observations:-
///
/// * Given the history of `SRV`, it seems prudent to permit compression.
/// * Given the similarity of `DNAME` to `CNAME`, and the historic confusion, it seems prudent to permit compression;
///
/// This gives a list of
///
/// * `CNAME`
/// * `MX`
/// * `NS`
/// * `PTR`
/// * `SOA`
/// * `NAPTR`
/// * `SRV`
/// * `DNAME`
pub(crate) struct ParsedNameParser<'a>
{
	compressed_name_presence_error: Option<ParsedNameParserError>,
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
	pub(crate) fn parse_name<'message>(compressed_name_presence_error: Option<ParsedNameParserError>, parsed_names: &'a mut ParsedNames, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<(ParsedName<'message>, usize), ParsedNameParserError>
	{
		let mut this = Self::new(compressed_name_presence_error, parsed_names, start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		this.parse()
	}
	
	#[inline(always)]
	pub(crate) fn parse_name_uncompressed<'message>(parsed_names: &mut ParsedNames, start_of_name_pointer: usize, resource_data_end_pointer: usize) -> Result<(ParsedName<'message>, usize), ParsedNameParserError>
	{
		Self::parse_name(Some(ParsedNameParserError::CompressedNameLabelsAreDisallowedInThisResourceRecord(data_type)), parsed_names, start_of_name_pointer, resource_data_end_pointer)
	}
	
	#[inline(always)]
	pub(crate) fn parse_name_in_slice_with_nothing_left<'message>(data_type: DataType, parsed_names: &mut ParsedNames, slice: &'message [u8]) -> Result<ParsedName<'message>, ParsedNameParserError>
	{
		let (parsed_name, end_of_name_pointer) = Self::parse_name_in_slice(data_type, parsed_names, slice)?;
		
		let end_of_data_section_containing_name_pointer = (unsafe { slice.as_ptr().add(slice.len()) }) as usize;
		if likely!(end_of_data_section_containing_name_pointer == end_of_name_pointer)
		{
			Ok(parsed_name)
		}
		else
		{
			Err(ParsedNameParserError::NameWasNotLongEnough)
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_name_in_slice<'message>(data_type: DataType, parsed_names: &mut ParsedNames, slice: &'message [u8]) -> Result<(ParsedName<'message>, usize), ParsedNameParserError>
	{
		let start_of_name_pointer = slice.as_ptr() as usize;
		let end_of_data_section_containing_name_pointer = (unsafe { slice.as_ptr().add(slice.len()) }) as usize;
		Self::parse_name(Some(ParsedNameParserError::CompressedNameLabelsAreDisallowedInThisResourceRecord(data_type)), parsed_names, start_of_name_pointer, end_of_data_section_containing_name_pointer)
	}
	
	#[inline(always)]
	fn new(compressed_name_presence_error: Option<ParsedNameParserError>, parsed_names: &'a mut ParsedNames, start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<Self, ParsedNameParserError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer {} occurs before start_of_name_pointer {}", end_of_data_section_containing_name_pointer, start_of_name_pointer);
		
		Ok
		(
			Self
			{
				compressed_name_presence_error,
				parsed_names,
				start_of_name_pointer,
				maximum_for_end_of_name_pointer: Self::maximum_for_end_of_name_pointer(start_of_name_pointer, end_of_data_section_containing_name_pointer)?,
				label_data_starts_at_pointers_and_label_length_excluding_trailing_period: ArrayVec::new(),
				name_length_including_trailing_periods_after_labels: 0,
			}
		)
	}
	
	#[inline(always)]
	fn parse<'message>(mut self) -> Result<(ParsedName<'message>, usize), ParsedNameParserError>
	{
		let mut label_starts_at_pointer = self.start_of_name_pointer;
		let end_of_qname_pointer = loop
		{
			match self.parse_label(label_starts_at_pointer)?
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
	fn parse_label(&mut self, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, ParsedNameParserError>
	{
		use self::LabelKind::*;
		use self::ParsedNameParserError::*;
		
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
	fn parse_bytes_label_kind(&mut self, raw_label: &RawLabel, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, ParsedNameParserError>
	{
		use self::ParsedNameParserError::*;
		
		let label_length_excluding_trailing_period = raw_label.bytes_length();
		let label_length_including_trailing_period = label_length_excluding_trailing_period + Self::SizeOfTrailingPeriod;
		self.name_length_including_trailing_periods_after_labels.checked_add(label_length_including_trailing_period).ok_or(LabelPointerCreatesADnsNameLongerThan255Bytes)?;
		
		let (label_data_starts_at_pointer, next_label_starts_at_pointer) = self.validate_label_length_does_not_cause_overflow(label_starts_at_pointer, label_length_excluding_trailing_period)?;
		
		Self::validate_label_does_not_contain_a_period(label_data_starts_at_pointer, next_label_starts_at_pointer);
		
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
	fn parse_compressed_label_kind(&mut self, raw_label: &RawLabel, label_starts_at_pointer: usize) -> Result<Either<usize, usize>, ParsedNameParserError>
	{
		use self::ParsedNameParserError::*;
		
		if let Some(compressed_name_presence_error) = self.compressed_name_presence_error
		{
			return Err(compressed_name_presence_error)
		}
		
		let compressed_pointer_offset = raw_label.compressed_pointer_offset();
		let parsed_name = self.parsed_names.look_up(compressed_pointer_offset, label_starts_at_pointer)?;
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
	
	#[inline(always)]
	fn validate_label_length_does_not_cause_overflow(&self, label_starts_at_pointer: usize, label_length_excluding_trailing_period: u8) -> Result<(usize, usize), ParsedNameParserError>
	{
		let label_data_starts_at = label_starts_at_pointer + LabelKind::BytesLabelKindSize;
		let next_label_starts_at_pointer = label_data_starts_at + (label_length_excluding_trailing_period as usize);
		if likely!(next_label_starts_at_pointer <= self.maximum_for_end_of_name_pointer)
		{
			Ok((label_data_starts_at, next_label_starts_at_pointer))
		}
		else
		{
			Err(ParsedNameParserError::LabelLengthOverflows)
		}
	}
	
	/// In theory, a label can actually contain a period.
	/// In practice, this is almost certainly either misconfiguration or a deliberate attempt to attack a code vulnerability.
	#[inline(always)]
	fn validate_label_does_not_contain_a_period(label_data_starts_at_pointer: usize, next_label_starts_at_pointer: usize) -> Result<(), ParsedNameParserError>
	{
		let mut pointer = label_data_starts_at_pointer;
		while pointer < next_label_starts_at_pointer
		{
			let byte = pointer.dereference_u8();
			if unlikely!(byte == b'.')
			{
				return Err(ParsedNameParserError::LabelContainsPeriod)
			}
			
			pointer += 1;
		}
		Ok(())
	}

	#[inline(always)]
	fn maximum_for_end_of_name_pointer(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, ParsedNameParserError>
	{
		let maximum_potential_name_length = Self::maximum_potential_name_length(start_of_name_pointer, end_of_data_section_containing_name_pointer)?;
		let maximum_for_end_of_name_pointer = start_of_name_pointer + maximum_potential_name_length;
		Ok(maximum_for_end_of_name_pointer)
	}

	#[inline(always)]
	fn maximum_potential_name_length(start_of_name_pointer: usize, end_of_data_section_containing_name_pointer: usize) -> Result<usize, ParsedNameParserError>
	{
		debug_assert!(end_of_data_section_containing_name_pointer >= start_of_name_pointer, "end_of_data_section_containing_name_pointer {} occurs before start_of_name_pointer {}", end_of_data_section_containing_name_pointer, start_of_name_pointer);

		if unlikely!(start_of_name_pointer == end_of_data_section_containing_name_pointer)
		{
			return Err(ParsedNameParserError::NameIsEmpty)
		}

		let unconstrained_maximum_potential_name_length = end_of_data_section_containing_name_pointer - start_of_name_pointer;
		Ok(min(unconstrained_maximum_potential_name_length, ParsedNameParser::NameMaximumSize))
	}
}
