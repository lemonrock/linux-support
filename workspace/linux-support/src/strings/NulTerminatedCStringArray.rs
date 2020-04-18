// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array of c strings (nul terminated byte arrays) with a final nul.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NulTerminatedCStringArray(Vec<u8>);

impl NulTerminatedCStringArray
{
	/// To pointer.
	#[inline(always)]
	pub fn as_ptr(&self) -> *const *const c_char
	{
		self.0.as_ptr() as *const _
	}

	/// `c_string_components` provides the pieces that are to create a CString.
	#[inline(always)]
	pub fn new<'a>(c_strings_as_fragments_to_combine: impl Iterator<Item=impl CStringFragments>) -> Self
	{
		let mut this = Self(Vec::with_capacity(PageSize::current().into()));

		for c_string_as_fragments_to_combine in c_strings_as_fragments_to_combine
		{
			c_string_as_fragments_to_combine.iterate(&mut |bytes| this.push_fragment(bytes));
			this.push_ascii_null();
		}
		this.push_ascii_null();
		this.0.shrink_to_fit();
		this
	}

	#[inline(always)]
	fn push_fragment(&mut self, bytes: &[u8])
	{
		debug_assert!(memchr(b'\0', bytes).is_none(), "Embedded ASCII NUL");

		let bytes_length = bytes.len();
		self.0.reserve(bytes_length);

		let original_buffer_length = self.0.len();
		unsafe
		{
			self.0.as_mut_ptr().add(original_buffer_length).copy_from_nonoverlapping(bytes.as_ptr(), bytes_length);
			self.0.set_len(original_buffer_length + bytes_length);
		}
	}

	#[inline(always)]
	fn push_ascii_null(&mut self)
	{
		self.0.push(b'\0')
	}
}
