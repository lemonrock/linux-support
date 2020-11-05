// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterates over character strings.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedCharacterStringsIterator<'message>
{
	next_string_starts_at_pointer: usize,
	end_of_resource_data_pointer: usize,
	marker: PhantomData<&'message ()>,
}

impl<'message> Iterator for ParsedCharacterStringsIterator<'message>
{
	type Item = Result<ParsedCharacterString<'message>, ParsedCharacterStringLengthIncorrectError>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.next_string_starts_at_pointer == self.end_of_resource_data_pointer)
		{
			return None
		}

		let text_string = self.next_string_starts_at_pointer.unsafe_cast::<RawCharacterString>();
		self.next_string_starts_at_pointer += 1;

		let length = text_string.length as usize;
		let result = if unlikely!(self.next_string_starts_at_pointer + length > self.end_of_resource_data_pointer)
		{
			Err(ParsedCharacterStringLengthIncorrectError)
		}
		else
		{
			Ok(ParsedCharacterString(text_string.as_slice(length)))
		};
		Some(result)
	}
}

impl<'message> ParsedCharacterStringsIterator<'message>
{
	#[inline(always)]
	pub(crate) fn new(resource_data: &'message [u8]) -> Result<Self, NoCharacterStringsError>
	{
		let length = resource_data.len();
		if unlikely!(length == 0)
		{
			return Err(NoCharacterStringsError)
		}

		let next_string_starts_at_pointer = resource_data.as_ptr() as usize;

		Ok
		(
			Self
			{
				next_string_starts_at_pointer,
				end_of_resource_data_pointer: next_string_starts_at_pointer + length,
				marker: PhantomData,
			}
		)
	}

	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.end_of_resource_data_pointer - self.next_string_starts_at_pointer == 0
	}

	#[inline(always)]
	pub(crate) fn remaining_resource_data(&'message self) -> &'message [u8]
	{
		self.next_string_starts_at_pointer.unsafe_cast_slice::<u8>(self.end_of_resource_data_pointer - self.next_string_starts_at_pointer)
	}
}
