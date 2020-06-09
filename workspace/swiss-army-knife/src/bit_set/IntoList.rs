// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Into a list.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IntoList<'a, BSA: BitSetAware>(pub &'a BitSet<BSA>);

impl<'a, BSA: BitSetAware> IntoLineFeedTerminatedByteString<'a> for IntoList<'a, BSA>
{
	/// To a list such as `1-3,5`.
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let mut string = String::with_capacity(4096);

		let mut written_first = false;
		let mut start_of_range_and_last_contiguous_value_in_range: Option<(u16, u16)> = None;
		for bit in self.0.iterate()
		{
			let current: u16 = bit.into();
			match start_of_range_and_last_contiguous_value_in_range
			{
				None =>
				{
					start_of_range_and_last_contiguous_value_in_range = Some((current, current));
				}

				Some((start_of_range, last_contiguous_value_in_range)) =>
				{
					if current - 1 == last_contiguous_value_in_range
					{
						start_of_range_and_last_contiguous_value_in_range = Some((start_of_range, current));
						continue
					}

					if written_first
					{
						string.push(',')
					}
					else
					{
						written_first = true
					}
					if start_of_range == last_contiguous_value_in_range
					{
						string.push_str(&format!("{}", start_of_range));
					}
					else
					{
						string.push_str(&format!("{}-{}", start_of_range, last_contiguous_value_in_range));
					}
					start_of_range_and_last_contiguous_value_in_range = Some((current, current));
				}
			}
		}

		if let Some((start_of_range, last_contiguous_value_in_range)) = start_of_range_and_last_contiguous_value_in_range
		{
			if written_first
			{
				string.push(',')
			}
			if start_of_range == last_contiguous_value_in_range
			{
				string.push_str(&format!("{}", start_of_range));
			}
			else
			{
				string.push_str(&format!("{}-{}", start_of_range, last_contiguous_value_in_range));
			}
		}

		string.push('\n');
		Cow::from(string.into_bytes())
	}
}
