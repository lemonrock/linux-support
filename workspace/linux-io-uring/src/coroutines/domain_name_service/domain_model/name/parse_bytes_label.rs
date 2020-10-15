// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! parse_bytes_label
{
	($length: ident, $current_label_starts_at_pointer: ident, $maximum_for_end_of_name_pointer: ident, $number_of_labels: ident, $name_length: ident) =>
	{
		{
			$number_of_labels += 1;

			let is_terminal_root_label = $length == 0;
			if unlikely!(is_terminal_root_label)
			{
				break $current_label_starts_at_pointer + LabelKind::LabelKindSize
			}

			let next_label_starts_at_pointer = guard_next_label_starts_at_pointer!($current_label_starts_at_pointer, $length, $maximum_for_end_of_name_pointer);

			/// NOTE: There are more efficient conversions of ASCII case for longer, aligned strings using AVX intrinsics, see https://stackoverflow.com/questions/735204/convert-a-string-in-c-to-upper-case/37151084#37151084 and https://gist.github.com/pcordes/815c3ed8752a24c64d427bcbfd1ee1c3.
			/// Most labels are short and not aligned.
			#[inline(always)]
			fn convert_to_ascii_lower_case(current_label_starts_at_pointer: usize, next_label_starts_at_pointer: usize)
			{
				for index in current_label_starts_at_pointer + LabelKind::LabelKindSize .. next_label_starts_at_pointer
				{
					let ascii_character = index.dereference_u8();

					if unlikely!(b'A' <= ascii_character && ascii_character <= b'Z')
					{
						index.set_u8(ascii_character + 0x20)
					}
				}
			}

			convert_to_ascii_lower_case($current_label_starts_at_pointer, next_label_starts_at_pointer);

			debug_assert!($length <= MaximumSizeOfLabelExcludingTrailingPeriod, "$label `{}` exceeds MaximumSizeOfLabelExcludingTrailingPeriod `{}`", $length, MaximumSizeOfLabelExcludingTrailingPeriod);

			$name_length += ($length as u8) + SizeOfTrailingPeriod;

			next_label_starts_at_pointer
		}
	}
}
