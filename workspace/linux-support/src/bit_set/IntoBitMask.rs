// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct IntoBitMask<'a, BSA: BitSetAware>(pub(crate) &'a BitSet<BSA>);

impl<'a, BSA: BitSetAware> IntoLineFeedTerminatedByteString<'a> for IntoBitMask<'a, BSA>
{
	/// To a bit mask such as `000000ff,11223344`.
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		const SizeOf8ByteHexadecimalTuple: usize = 8;
		const SizeOfLineFeed: usize = 1;

		let capacity = self.0.capacity();
		let number_of_bytes = (capacity * SizeOf8ByteHexadecimalTuple) + (capacity - 1) + SizeOfLineFeed;
		let mut bytes = Vec::with_capacity(number_of_bytes);
		unsafe { bytes.set_len(number_of_bytes) };

		bytes[number_of_bytes - 1] = b'\n';
		let mut bytes_index = number_of_bytes - 2;

		for word_index in (0 .. capacity).rev()
		{
			if likely!(word_index > 0)
			{
				bytes[bytes_index] = b',';
				bytes_index -= 1;
			}

			bytes_index = self.0.get_word(word_index).lower_case_hexadecimal(bytes_index, &mut bytes[..]);
		}

		Cow::from(bytes)
	}
}
