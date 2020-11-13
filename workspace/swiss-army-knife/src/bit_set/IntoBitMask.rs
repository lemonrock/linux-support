// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Into a bit mask.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IntoBitMask<'a, BSA: BitSetAware>(pub &'a BitSet<BSA>);

impl<'a, BSA: BitSetAware> IntoLineFeedTerminatedByteString<'a> for IntoBitMask<'a, BSA>
{
	/// To a bit mask such as `000000ff,11223344`.
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		const SizeOf8ByteHexadecimalTuple: usize = 8;
		const SizeOfLineFeed: usize = 1;

		let capacity = self.0.capacity_in_words();
		if unlikely!(capacity == 0)
		{
			return Cow::from(b"00000000\n" as &[u8])
		}


		let number_of_bytes = (capacity * SizeOf8ByteHexadecimalTuple) + (capacity - 1) + SizeOfLineFeed;
		let mut bytes = Vec::with_capacity(number_of_bytes);
		unsafe { bytes.set_len(number_of_bytes) };

		bytes[number_of_bytes - 1] = b'\n';
		let mut bytes_index = number_of_bytes - 2;

		let mut word_index = 0;
		let last_word_index = capacity - 1;

		// This logic writes out from right-to-left.
		loop
		{
			if likely!(word_index > 0)
			{
				bytes.set_unchecked_mut_safe(bytes_index, b',');
				bytes_index -= 1;
			}

			let bytes_index_of_lefthand_digit = self.0.get_word(word_index).lower_case_hexadecimal(bytes_index, &mut bytes[..]);

			if unlikely!(word_index == last_word_index)
			{
				debug_assert_eq!(bytes_index_of_lefthand_digit, 0);
				break;
			}
			word_index += 1;
			bytes_index = bytes_index_of_lefthand_digit - 1;
		}

		Cow::from(bytes)
	}
}
