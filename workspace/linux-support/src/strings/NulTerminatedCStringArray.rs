// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array of c strings (nul terminated byte arrays) with a final nul.
///
/// `buffer` contains a list of NUL-terminated C strings, each with a terminating NUL, before terminating the entire list with NUL (ie there are two NUL bytes at the end).
/// `pointers` points to C strings in `buffer`, with a final pointer of `null()`.
///
/// NOTE: Empty strings are allowed, but, if using only the `buffer`, they can not be discovered simply by walking the `buffer`'s bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NulTerminatedCStringArray
{
	buffer: Vec<u8>,
	pointers: Vec<*const c_char>,
}

impl NulTerminatedCStringArray
{
	/// `c_string_components` provides the pieces that are to create a CString.
	#[inline(always)]
	pub fn new<'a>(c_strings_as_fragments_to_combine: impl Iterator<Item=impl CStringFragments>) -> Self
	{
		let estimated_number_of_strings =
		{
			let (lower, potential_upper) = c_strings_as_fragments_to_combine.size_hint();
			if let Some(upper) = potential_upper
			{
				upper
			}
			else
			{
				max(16, lower)
			}
		};

		let mut this = Self
		{
			buffer: Vec::with_capacity(PageSize::current().into()),
			pointers: Vec::with_capacity(estimated_number_of_strings),
		};

		for c_string_as_fragments_to_combine in c_strings_as_fragments_to_combine
		{
			this.push_c_string(c_string_as_fragments_to_combine)
		}

		this.finish();

		this
	}

	/// Array of pointers.
	#[inline(always)]
	pub fn as_array_of_pointers(&self) -> *const *const c_char
	{
		self.pointers.as_ptr()
	}

	/// Buffer, suitable for `/proc/<pid>/environ` and `/proc/<pid>/cmdline` if `preserve_final_nul_if_empty` is `false`.
	#[inline(always)]
	pub fn as_block_of_strings(&self, preserve_final_nul_if_empty: bool) -> &[u8]
	{
		if preserve_final_nul_if_empty
		{
			&self.buffer[..]
		}
		else
		{
			&self.buffer[0 .. self.buffer.len() - 1]
		}
	}

	#[inline(always)]
	fn push_fragment(&mut self, bytes: &[u8])
	{
		debug_assert!(memchr(b'\0', bytes).is_none(), "Embedded ASCII NUL");

		let bytes_length = bytes.len();
		self.buffer.reserve(bytes_length);

		let original_buffer_length = self.buffer.len();
		unsafe
		{
			self.buffer.as_mut_ptr().add(original_buffer_length).copy_from_nonoverlapping(bytes.as_ptr(), bytes_length);
			self.buffer.set_len(original_buffer_length + bytes_length);
		}
	}

	#[inline(always)]
	fn push_c_string(&mut self, c_string_as_fragments_to_combine: impl CStringFragments)
	{
		let relative_c_string_pointer = self.buffer.len();
		c_string_as_fragments_to_combine.iterate(&mut |bytes| self.push_fragment(bytes));
		self.push_end_of_c_string(relative_c_string_pointer);
	}

	#[inline(always)]
	fn push_end_of_c_string(&mut self, relative_c_string_pointer: usize)
	{
		self.push_ascii_null();
		self.push_pointer(unsafe { self.buffer.as_ptr().add(relative_c_string_pointer) as *const c_char });
	}

	#[inline(always)]
	fn finish(&mut self)
	{
		self.push_ascii_null();
		self.push_pointer(null());

		self.buffer.shrink_to_fit();
		self.pointers.shrink_to_fit();
	}

	#[inline(always)]
	fn push_ascii_null(&mut self)
	{
		self.buffer.push(b'\0');
	}

	#[inline(always)]
	fn push_pointer(&mut self, pointer: *const c_char)
	{
		self.pointers.push(pointer);
	}
}
