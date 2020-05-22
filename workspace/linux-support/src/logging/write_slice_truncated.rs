// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
unsafe fn write_slice_truncated(write_to: *mut u8, slice: &[u8], end_pointer: *mut u8) -> *mut u8
{
	let length = slice.len();
	let slice = if write_to.add(length) > end_pointer
	{
		&slice[0 .. (end_pointer as usize) - (write_to as usize)]
	}
	else
	{
		slice
	};
	write_slice_unchecked(write_to, slice, end_pointer)
}
