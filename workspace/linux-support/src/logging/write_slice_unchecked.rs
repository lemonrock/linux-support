// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
unsafe fn write_slice_unchecked(write_to: *mut u8, slice: &[u8], end_pointer: *mut u8) -> *mut u8
{
	let length = slice.len();
	debug_assert!(write_to.add(length) <= end_pointer);
	slice.as_ptr().copy_to_nonoverlapping(write_to, length);
	write_to.add(length)
}
