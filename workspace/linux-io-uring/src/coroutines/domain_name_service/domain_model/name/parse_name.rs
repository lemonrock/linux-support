// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! parse_name
{
	($start_of_name_pointer: ident, $end_of_data_section_containing_name_pointer: ident, $labels_register_reference: ident, $parsed_labels: ident, $bytes: ident, $compressed: ident) =>
	{
		{
			const MaximumSizeOfLabelExcludingTrailingPeriod: usize = 63;
			const SizeOfTrailingPeriod: u8 = 1;

			let maximum_for_end_of_name_pointer = Name::maximum_for_end_of_name_pointer($start_of_name_pointer, $end_of_data_section_containing_name_pointer)?;

			let mut pointer_to_label = $start_of_name_pointer;
			let mut current_label_starts_at_pointer = pointer_to_label;
			let mut number_of_labels: u8 = 0;
			let mut name_length: u8 = 0;

			let true_end_of_name_pointer = loop
			{
				if unlikely!(current_label_starts_at_pointer == maximum_for_end_of_name_pointer)
				{
					return Err(NoTerminalRootLabel)
				}
				let label = Label::label(current_label_starts_at_pointer);

				use self::LabelKind::*;
				match label.raw_kind()
				{
					Bytes =>
					{
						current_label_starts_at_pointer = $bytes!(label, current_label_starts_at_pointer, maximum_for_end_of_name_pointer, $labels_register_reference, number_of_labels, name_length);
					}

					Extended => return Err(ExtendedNameLabelsAreUnused),

					Unallocated => return Err(UnallocatedNameLabelsAreUnused),

					CompressedOffsetPointer => $compressed!(label, current_label_starts_at_pointer, maximum_for_end_of_name_pointer, $start_of_name_pointer, $pointer_to_label, $labels_register_reference, $parsed_labels, number_of_labels, name_length),
				}
			};
			(pointer_to_label, true_end_of_name_pointer, number_of_labels, name_length)
		}
	}
}
