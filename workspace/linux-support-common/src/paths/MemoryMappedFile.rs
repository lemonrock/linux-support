// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory-mapped file.
#[derive(Debug)]
pub struct MemoryMappedFile
{
	/// Technically, this isn't true; `mmap()` could return a pointer of zero.
	pointer: NonNull<u8>,
	size: usize,
}

impl Drop for MemoryMappedFile
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { munmap(self.pointer.as_ptr() as *mut c_void, self.size) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			panic!("munmap returned an error of {}", errno())
		}
		else
		{
			panic!("munmap() failed with an unexpected exit code of {:?}", result)
		}
	}
}

impl MemoryMappedFile
{
	/// Get value.
	///
	/// `T: Copy` to force `T` to not implement `Drop` as otherwise a memory leak could occur.
	#[inline(always)]
	pub fn get<T: Copy>(&self, offset: usize) -> T
	{
		let pointer = self.pointer_to(offset).as_ptr();
		unsafe { read_volatile(pointer) }
	}

	/// Set value.
	///
	/// `T: Copy` to force `T` to not implement `Drop` as otherwise a memory leak could occur.
	#[inline(always)]
	pub fn set<T: Copy>(&self, offset: usize, value: T)
	{
		let pointer = self.pointer_to(offset).as_ptr();
		unsafe { write_volatile(pointer, value) }
	}

	/// Pointer to value.
	#[inline(always)]
	fn pointer_to<T>(&self, offset: usize) -> NonNull<T>
	{
		debug_assert_eq!(offset % align_of::<T>(), 0, "misaligned access");

		unsafe { NonNull::new_unchecked(((self.pointer.as_ptr() as usize) + offset) as *mut T) }
	}
}
