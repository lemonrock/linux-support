// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A subrange.
#[derive(Debug)]
pub struct MappedMemorySubrange<'a, Subrange: RelativeMemoryRange>(&'a MappedMemory, Subrange);

impl<'a, Subrange: RelativeMemoryRange> From<(&'a MappedMemory, Subrange)> for MappedMemorySubrange<'a, Subrange>
{
	#[inline(always)]
	fn from(from: (&'a MappedMemory, Subrange)) -> Self
	{
		Self(from.0, from.1)
	}
}

impl<'a, Subrange: RelativeMemoryRange> AbsoluteMemoryRange for MappedMemorySubrange<'a, Subrange>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		self.0.sub_range_inner(&self.1)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.0.sub_range_inner(&self.1).0
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.0.sub_range_inner(&self.1).1
	}
}

impl<'a: 'b, 'b, Subrange: RelativeMemoryRange> AbsoluteMemoryRange for &'b MappedMemorySubrange<'a, Subrange>
{
	#[inline(always)]
	fn inclusive_absolute_start_and_length(self) -> (VirtualAddress, usize)
	{
		self.0.sub_range_inner(&self.1)
	}
	
	#[inline(always)]
	fn inclusive_absolute_start(self) -> VirtualAddress
	{
		self.0.sub_range_inner(&self.1).0
	}
	
	#[inline(always)]
	fn length(self) -> usize
	{
		self.0.sub_range_inner(&self.1).1
	}
}

impl<'a, Subrange: RelativeMemoryRange + Copy> MappedMemorySubrange<'a, Subrange>
{
	/// Returns `Ok(true)` if memory was locked.
	/// Returns `Ok(false)` if only some (or none) of memory was locked but locking can be retried.
	#[inline(always)]
	pub fn lock(&self, memory_lock_settings: MemoryLockSettings) -> io::Result<bool>
	{
		self.0.lock_range(memory_lock_settings, self.1)
	}
	
	/// Returns `Ok(true)` if memory was unlocked.
	/// Returns `Ok(false)` if only some (or none) of memory was unlocked but unlocking can be retried.
	#[inline(always)]
	pub fn unlock(&self) -> io::Result<bool>
	{
		self.0.unlock_range(self.1)
	}
	
	/// Advise Linux kernel of usage of this memory.
	///
	/// If the Linux kernel wasn't compiled with `CONFIG_ADVISE_SYSCALLS`, this system call will fail.
	#[inline(always)]
	pub fn advise(&self, advice: MemoryAdvice) -> io::Result<()>
	{
		self.0.advise_range(advice, self.1)
	}
	
	/// Does not support the obsolete `PROT_SEM` flag.
	/// Does not support combining the PowerPC-only `PROT_SAO` flag with other flags to minimize syscalls, sorry.
	/// Does not support combining the deprecated `PROT_GROWSUP` and `PROT_GROWSDOWN` flags with other flags to minimize syscalls, sorry.
	#[inline(always)]
	pub fn change_protection(&self, protection: ExtendedProtection) -> io::Result<()>
	{
		self.0.change_protection_range(protection, self.1)
	}
	
	/// Synchronize a file-backed mapping.
	///
	/// Returns `Err()` if `synchronize` asked to invalidate and a memory lock exists which covers all or part of `self`.
	#[inline(always)]
	pub fn synchronize_with_backing_file(&self, synchronize: SynchronizeFlags) -> Result<(), ()>
	{
		self.0.synchronize_with_backing_file_range(synchronize, self.1)
	}
	
	/// Zeros range.
	#[inline(always)]
	pub fn zero(&self)
	{
		self.0.zero_range(self.1)
	}
	
	/// Owns this reference.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn owns_reference<E>(&self, reference: &E) -> bool
	{
		self.owns_non_null(new_non_null(reference as *const E as *mut E as *mut u8))
	}
	
	/// Owns this non-null.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn owns_non_null<E>(&self, non_null: NonNull<E>) -> bool
	{
		self.owns_pointer(non_null.as_ptr() as *const E)
	}
	
	/// Owns this pointer.
	///
	/// Treats memory as a `Range`, so does not own the pointer if it is equal to `self.virtual_address() + self.mapped_size_in_bytes()`.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn owns_pointer<E>(&self, pointer: *const E) -> bool
	{
		let (start, length) = self.inclusive_absolute_start_and_length();
		
		pointer as *const u8 as usize;
		let start: *const E = start.into();
		if unlikely!(start > pointer)
		{
			return false
		}
		let end = self.virtual_address().offset_in_bytes(length).into();
		pointer < end
	}
	
	/// Virtual address.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn virtual_address(&self) -> VirtualAddress
	{
		self.inclusive_absolute_start()
	}
	
	/// Mapped page size used.
	#[inline(always)]
	pub fn page_size(&self) -> PageSizeOrHugePageSize
	{
		self.0.page_size()
	}
	
	/// Mapped size in bytes.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn mapped_size_in_bytes(&self) -> usize
	{
		self.length()
	}
	
	/// Mapped size in number of pages.
	///
	/// This method may be inefficient.
	#[inline(always)]
	pub fn number_of_pages(&self) -> usize
	{
		self.mapped_size_in_bytes() / (self.0.page_size_in_bytes().get() as usize)
	}
}
