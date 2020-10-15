// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! guard_next_label_starts_at_pointer
{
	($current_label_starts_at_pointer: ident, $length: ident, $maximum_for_end_of_name_pointer: ident) =>
	{
		{
			let next_label_starts_at_pointer = $current_label_starts_at_pointer + LabelKind::LabelKindSize + $length;
			if unlikely!(next_label_starts_at_pointer > $maximum_for_end_of_name_pointer)
			{
				return Err(LabelLengthOverflows)
			}

			next_label_starts_at_pointer
		}
	}
}
