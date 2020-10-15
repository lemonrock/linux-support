// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! compressed_error
{
	($_current_label_starts_at_pointer: ident, $_maximum_for_end_of_name_pointer: ident, $_label: ident, $_start_of_name_pointer: ident, $_pointer_to_label: ident, $_labels_register_reference: ident, $_parsed_labels: ident, $_number_of_labels: ident, $name_length: ident) =>
	{
		{
			return Err(CompressedNameLabelsAreDisallowedInThisResourceRecord)
		}
	}
}
