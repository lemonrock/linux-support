// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CString extensions.
pub trait CStringExt
{
	/// From a fixed length buffer, where the string is *always* terminated by an ASCII NUL.
	fn from_fixed_length_buffer_ascii_nul_terminated(buffer: Vec<c_char>) -> CString;

	/// From a fixed length buffer, where the string is not terminated by an ASCII NUL if it occupies the entire buffer.
	fn from_fixed_length_buffer_optionally_ascii_nul_terminated(buffer: Vec<c_char>) -> CString;
}

impl CStringExt for CString
{
	#[inline(always)]
	fn from_fixed_length_buffer_ascii_nul_terminated(mut buffer: Vec<c_char>) -> CString
	{
		let index = memchr(b'\0', unsafe { transmute(&buffer[..]) }).expect("final element was not ASCII null");
		unsafe { buffer.set_len(index + 1) };
		let c_string_inner = buffer.into_boxed_slice();
		unsafe { transmute(c_string_inner) }
	}

	#[inline(always)]
	fn from_fixed_length_buffer_optionally_ascii_nul_terminated(mut buffer: Vec<c_char>) -> CString
	{
		match memchr(b'\0', unsafe { transmute(&buffer[..]) })
		{
			Some(index) =>
			{
				unsafe { buffer.set_len(index + 1) };
			}
			None =>
			{
				buffer.push(b'\0' as i8)
			}
		};
		let c_string_inner = buffer.into_boxed_slice();
		unsafe { transmute(c_string_inner) }
	}
}
