// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended attribute names.
///
/// Names might start with a namespace followed by a period (`.`), including:-
///
/// * `security`
/// * `system`
/// * `trusted`
/// * `user`
#[derive(Debug)]
pub struct ExtendedAttributeNames<'a>
{
	buffer: Vec<u8>,
	next_name_inclusive_starts_at: usize,
	marker: PhantomData<&'a ()>,
}

impl<'a> Iterator for ExtendedAttributeNames<'a>
{
	type Item = ExtendedAttributeName<'a>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		if unlikely!(self.next_name_inclusive_starts_at == self.buffer.len())
		{
			return None
		}

		let ends_at_index = (&self.buffer[self.next_name_inclusive_starts_at .. ]).memchr(b'\0').expect("Bug in data from Linux kernel; lacks terminating ASCII NUL");
		let inclusive_ends_at = ends_at_index + 1;
		let bytes_with_nul = &self.buffer[self.next_name_inclusive_starts_at .. inclusive_ends_at];

		self.next_name_inclusive_starts_at = inclusive_ends_at;

		Some(ExtendedAttributeName(unsafe { &*(bytes_with_nul as *const [u8] as *const CStr) }))
	}
}

impl<'a> ExtendedAttributeNames<'a>
{
	#[inline(always)]
	pub(crate) const fn new(buffer: Vec<u8>) -> Self
	{
		Self
		{
			buffer,
			next_name_inclusive_starts_at: 0,
			marker: PhantomData,
		}
	}

	/// Rewind.
	#[inline(always)]
	pub fn rewind(&mut self)
	{
		self.next_name_inclusive_starts_at = 0
	}
}
