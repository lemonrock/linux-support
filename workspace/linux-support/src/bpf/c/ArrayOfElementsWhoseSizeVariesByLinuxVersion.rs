// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An array that would be a slice if the element size was constant!
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayOfElementsWhoseSizeVariesByLinuxVersion
{
	pointer: NonNull<u8>,
	
	number_of_elements: usize,
	
	element_size_for_this_version_of_linux: usize,
}

impl ArrayOfElementsWhoseSizeVariesByLinuxVersion
{
	/// Length.
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.number_of_elements
	}
	
	/// Is it compatible with the layout of the struct `V`?
	#[inline(always)]
	pub fn is_compatible_with<V: Sized>(&self) -> bool
	{
		self.element_size_for_this_version_of_linux <= size_of::<V>()
	}
	
	/// Get, unchecked.
	#[inline(always)]
	pub unsafe fn get_unchecked(&self, index: usize) -> &[u8]
	{
		debug_assert!(index < self.number_of_elements);
		
		unsafe { from_raw_parts(self.pointer.as_ptr().add(index * self.element_size_for_this_version_of_linux), self.element_size_for_this_version_of_linux) }
	}
}
