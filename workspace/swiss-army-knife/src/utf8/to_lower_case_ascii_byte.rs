// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[inline(always)]
pub const fn to_lower_case_ascii_byte(upper_case_ascii_byte: u8) -> u8
{
	debug_assert!(upper_case_ascii_byte >= A && upper_case_ascii_byte <= Z);
	
	/// If 6th bit set ASCII byte is upper case.
	const ASCII_CASE_MASK: u8 = 0b0010_0000;
	upper_case_ascii_byte ^ ASCII_CASE_MASK
}
