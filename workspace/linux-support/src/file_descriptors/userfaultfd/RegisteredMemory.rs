// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registered memory.
///
/// Higher-level API than that of `UserFaultFileDescriptor`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisteredMemory
{
	very_large_range_of_mapped_memory: ManuallyDrop<MappedMemory>,
	
	user_fault_file_descriptor: Arc<UserFaultFileDescriptor>,
}

impl Drop for RegisteredMemory
{
	#[cold]
	fn drop(&mut self)
	{
		let _swallow_error = self.user_fault_file_descriptor().unregister_memory_subrange(& * self.very_large_range_of_mapped_memory);
		
		unsafe { ManuallyDrop::drop(&mut self.very_large_range_of_mapped_memory) };
	}
}

impl AsRef<Arc<UserFaultFileDescriptor>> for RegisteredMemory
{
	#[inline(always)]
	fn as_ref(&self) -> &Arc<UserFaultFileDescriptor>
	{
		&self.user_fault_file_descriptor
	}
}

impl RegisteredMemory
{
	/// Create a new instance.
	#[cold]
	pub fn new(user_fault_file_descriptor: Arc<UserFaultFileDescriptor>, number_of_pages: NonZeroUsize) -> Result<Self, RegisteredMemoryCreationError>
	{
		let length = PageSizeOperations::number_of_bytes_from_number_of_pages(number_of_pages.get());
		
		let very_large_range_of_mapped_memory = Self::very_large_range_of_mapped_memory(length)?;
		let supported_input_output_control_requests_for_registered_memory = user_fault_file_descriptor.register_memory_subrange(&very_large_range_of_mapped_memory, PageFaultEventNotificationSetting::BothIfMissingAndIfWriteProtectedPageAccess).map_err(RegisteredMemoryCreationError::Registration)?;
		
		assert!(supported_input_output_control_requests_for_registered_memory.contains(SupportedInputOutputControlRequests::RegularPagesWithWriteProtectOnCopy), "supported_input_output_control_requests {:?} lacking basic essential ioctls {:?}", supported_input_output_control_requests_for_registered_memory, SupportedInputOutputControlRequests::RegularPagesWithWriteProtectOnCopy);
		
		Ok
		(
			Self
			{
				very_large_range_of_mapped_memory: ManuallyDrop::new(very_large_range_of_mapped_memory),
			
				user_fault_file_descriptor,
			}
		)
	}
	
	/// Returns memory to the Linux Kernel using `madvise(MADV_FREE)` by setting the dirty bit in the page table entry (PTE).
	///
	/// Trying to read or write to the requested memory range after this call *may* cause a user page fault.
	///
	/// The user page fault will only occur if the memory has subsequently been used for something else by the Linux kernel.
	#[inline(always)]
	pub fn soft_remove_from_user_faulted_memory(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize) -> Result<(), MemoryAdviceError>
	{
		self.advise(our_virtual_address, number_of_pages, MemoryAdvice::Free)
	}
	
	/// Returns memory to the Linux Kernel using `madvise(MADV_DONTNEED)`.
	///
	/// Trying to read or write to the requested memory range after this call will cause a user page fault.
	///
	/// Expensive.
	#[inline(always)]
	pub fn hard_remove_from_user_faulted_memory(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize) -> Result<(), MemoryAdviceError>
	{
		self.advise(our_virtual_address, number_of_pages, MemoryAdvice::DontNeed)
	}
	
	/// Copy to a range of registered memory that has yet to be 'faulted in'.
	///
	/// `copy_starting_from` does not need to be registered memory.
	#[inline(always)]
	pub fn copy(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool, write_protect: bool, copy_starting_from: VirtualAddress) -> Result<(), CopyError>
	{
		let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
		self.user_fault_file_descriptor().copy_registered_memory_subrange(registered_memory_subrange, copy_starting_from, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange, write_protect)
	}
	
	/// Copy zero bytes to a range of registered memory that has yet to be 'faulted in'.
	#[inline(always)]
	pub fn zero_copy(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool, write_protect: bool) -> Result<(), CopyError>
	{
		if write_protect
		{
			if wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange
			{
				let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
				self.user_fault_file_descriptor().zero_registered_memory_subrange(registered_memory_subrange, false)?;
				self.user_fault_file_descriptor().enable_write_protection_of_registered_memory_subrange_and_wake_up_registered_memory_subrange(registered_memory_subrange).map_err(|()| CopyError::LinuxKernelIsOutOfMemory)
			}
			else
			{
				const PageAtATime: NonZeroUsize = new_non_zero_usize(1);
				
				let page_size_in_bytes = PageSize::default().into_usize();
				let end_virtual_address = our_virtual_address.offset_in_bytes(page_size_in_bytes * number_of_pages.get());
				let mut virtual_address = our_virtual_address;
				loop
				{
					self.copy(virtual_address, PageAtATime, false, true, PageSizeOperations::zero_page_for_write_protected_copies())?;
					
					virtual_address = virtual_address.offset_in_bytes(page_size_in_bytes);
					if unlikely!(virtual_address == end_virtual_address)
					{
						break
					}
				}
				Ok(())
			}
		}
		else
		{
			let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
			self.user_fault_file_descriptor().zero_registered_memory_subrange(registered_memory_subrange, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange)
		}
	}
	
	/// Wake up a thread or threads waiting on page fault resolution on a range of registered memory.
	#[inline(always)]
	pub fn wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize)
	{
		let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
		self.user_fault_file_descriptor().wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange(registered_memory_subrange)
	}
	
	/// Enable write protection (WP) on a range of registered memory.
	#[inline(always)]
	pub fn enable_write_protection_and_wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize) -> Result<(), ()>
	{
		let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
		self.user_fault_file_descriptor().enable_write_protection_of_registered_memory_subrange_and_wake_up_registered_memory_subrange(registered_memory_subrange)
	}
	
	/// Disable write protection (WP) on a range of registered memory.
	#[inline(always)]
	pub fn disable_write_protection(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange: bool) -> Result<(), ()>
	{
		let registered_memory_subrange = self.registered_memory_subrange_checked(our_virtual_address, number_of_pages);
		self.user_fault_file_descriptor().disable_write_protection_of_registered_memory_subrange(registered_memory_subrange, wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange)
	}
	
	#[inline(always)]
	fn advise(&self, our_virtual_address: VirtualAddress, number_of_pages: NonZeroUsize, advice: MemoryAdvice) -> Result<(), MemoryAdviceError>
	{
		loop
		{
			let succeeded = self.very_large_range_of_mapped_memory.advise_range(advice, self.mapped_memory_subrange_checked(our_virtual_address, number_of_pages))?;
			if likely!(succeeded)
			{
				return Ok(())
			}
		}
		
	}
	
	#[inline(always)]
	fn mapped_memory_subrange_checked(&self, virtual_address: VirtualAddress, number_of_pages: NonZeroUsize) -> Range<usize>
	{
		self.registered_memory_subrange_checked(virtual_address, number_of_pages).into_range(self.start_virtual_address())
	}
	
	#[inline(always)]
	fn registered_memory_subrange_checked(&self, virtual_address: VirtualAddress, number_of_pages: NonZeroUsize) -> FastAbsoluteMemoryRange
	{
		let number_of_bytes_in_subrange = Self::number_of_bytes_from_number_of_pages(number_of_pages);
		
		if cfg!(debug_assertions)
		{
			debug_assert!(self.start_virtual_address() <= virtual_address);
			debug_assert!(self.end_virtual_address() >= virtual_address.offset_in_bytes(number_of_bytes_in_subrange));
		}
		
		FastAbsoluteMemoryRange::new(virtual_address, number_of_bytes_in_subrange)
	}
	
	#[cfg(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64"))]
	#[inline(always)]
	const fn number_of_bytes_from_number_of_pages(number_of_pages: NonZeroUsize) -> usize
	{
		PageSizeOperations::number_of_bytes_from_number_of_pages(number_of_pages.get())
	}
	
	#[cfg(not(any(target_arch = "powerpc64", target_arch = "riscv64", target_arch = "sparc64", target_arch = "x86_64")))]
	#[inline(always)]
	fn number_of_bytes_from_number_of_pages(number_of_pages: NonZeroUsize) -> usize
	{
		PageSizeOperations::number_of_bytes_from_number_of_pages(number_of_pages.get())
	}
	
	#[inline(always)]
	fn start_virtual_address(&self) -> VirtualAddress
	{
		self.very_large_range_of_mapped_memory.virtual_address()
	}
	
	#[inline(always)]
	fn end_virtual_address(&self) -> VirtualAddress
	{
		self.start_virtual_address().offset_in_bytes(self.very_large_range_of_mapped_memory.mapped_size_in_bytes())
	}
	
	#[inline(always)]
	fn user_fault_file_descriptor(&self) -> &UserFaultFileDescriptor
	{
		self.user_fault_file_descriptor.deref()
	}
	
	#[inline(always)]
	fn very_large_range_of_mapped_memory(length: usize) -> Result<MappedMemory, RegisteredMemoryCreationError>
	{
		use self::RegisteredMemoryCreationError::*;
		
		let mapped_memory = MappedMemory::anonymous(new_non_zero_u64(length as u64), AddressHint::any(), Protection::ReadWrite, Sharing::Private, false, false, PageSizeOperations::default_page_size_or_huge_page_size_settings()).map_err(MappingMemory)?;
		
		use self::MemoryAdvice::*;
		const Advice: [MemoryAdvice; 3] =
		[
			DisableTransparentHugePages,
			DontFork,
			DontDump,
		];
		
		for advice in Advice.iter()
		{
			loop
			{
				let advice = *advice;
				let succeeded = mapped_memory.advise(advice).map_err(|cause| CouldNotAdvise(cause, advice))?;
				if likely!(succeeded)
				{
					break
				}
			}
		}
		
		Ok(mapped_memory)
	}
}
