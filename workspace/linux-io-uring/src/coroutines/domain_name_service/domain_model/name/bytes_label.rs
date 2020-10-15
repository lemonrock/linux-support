// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! bytes_label
{
	($self: ident, $label: ident, $pointer_to_label: ident) =>
	{
		{
			let length = $label.length();

			let after_label_kind_byte = $pointer_to_label + LabelKind::LabelKindSize;
			let label_bytes = Some(unsafe { from_raw_parts(after_label_kind_byte as *const u8, length) });
			$self.pointer_to_label = after_label_kind_byte + length;
			label_bytes
		}
	}
}
