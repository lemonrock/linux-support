// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


trait MultipleBits: Default + Copy + Into<tcflag_t>
{
	const Bitmask: tcflag_t = 0;

	#[inline(always)]
	fn from_mode_flags(mode_flags: tcflag_t) -> Self
	{
		Self::transmute_from_clean_mode_flags(mode_flags | Self::Bitmask)
	}

	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self;

	#[inline(always)]
	fn change_mode_flags(this: Option<Self>, current_flags: tcflag_t) -> tcflag_t
	{
		match this
		{
			None => current_flags,
			Some(multiple_bits) =>
			{
				let bits = multiple_bits.into();
				(current_flags & !Self::Bitmask) | bits
			},
		}
	}
}
