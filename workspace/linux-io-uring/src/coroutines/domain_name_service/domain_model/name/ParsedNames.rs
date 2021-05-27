// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedNames<'message>
{
	start_of_message_pointer: usize,
	
	parsed_names: HashMap<CompressedPointerOffset, ParsedName<'message>>,
}

impl<'message> ParsedNames<'message>
{
	#[inline(always)]
	pub(crate) fn new(start_of_message_pointer: usize) -> Self
	{
		Self
		{
			start_of_message_pointer,
		
			parsed_names: HashMap::new(),
		}
	}
	
	fn register(&mut self, label_data_starts_at_pointers_and_label_length_excluding_trailing_period: &ArrayVec<(usize, u8), LabelMaximumNumber>)
	{
		let mut name_length_including_trailing_period = 0;
		
		let mut length = label_data_starts_at_pointers_and_label_length_excluding_trailing_period.len();
		while length != 0
		{
			let index = length - 1;
			let (label_data_starts_at_pointer, label_length_excluding_trailing_period) = label_data_starts_at_pointers_and_label_length_excluding_trailing_period.get_unchecked_value_safe(index);
			let label_length_including_trailing_period = label_length_excluding_trailing_period + ParsedNameParser::SizeOfTrailingPeriod;
			
			name_length_including_trailing_period += label_length_including_trailing_period;
			
			if let Ok(compressed_pointer_offset) = CompressedPointerOffset::try_from(label_data_starts_at_pointer - self.start_of_message_pointer)
			{
				let was = self.parsed_names.insert
				(
					compressed_pointer_offset,
					ParsedName::new
					(
						{
							let mut new = ArrayVec::new();
							new.try_extend_from_slice(&label_data_starts_at_pointers_and_label_length_excluding_trailing_period[index .. ]).unwrap();
							new
						},
						name_length_including_trailing_period
					)
				);
				debug_assert!(was.is_none());
			}
			
			length -= 1;
		}
	}
	
	fn look_up(&self, compressed_pointer_offset: CompressedPointerOffset, label_starts_at_pointer: usize) -> Result<&ParsedName<'message>, ParsedNameParserError>
	{
		use self::ParsedNameParserError::*;
		
		// This check is an optimization to prevent the cost of a hash look up.
		if unlikely!(compressed_pointer_offset.start_of_name_pointer(self.start_of_message_pointer) >= label_starts_at_pointer)
		{
			return Err(LabelPointerPointsToDataAfterTheStartOfTheCurrentlyBeingParsedName)
		}
		
		self.parsed_names.get(&compressed_pointer_offset).ok_or(LabelPointerPointsToALabelThatWasNotPreviouslyParsed(compressed_pointer_offset))
	}
}
