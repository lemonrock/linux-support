// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory mapped using `mmap()`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MappedMemory
{
	virtual_address: VirtualAddress,
	size: usize,
	page_size: PageSizeOrHugePageSize,
}

impl Drop for MappedMemory
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unlikely!(self.size == 0)
		{
			return
		}
		self.unmap(self.size)
	}
}

impl Deref for MappedMemory
{
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { from_raw_parts(self.virtual_address.into(), self.size) }
	}
}

impl DerefMut for MappedMemory
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { from_raw_parts_mut(self.virtual_address.into(), self.size) }
	}
}

impl MappedMemory
{
	/// Creates a new mapping.
	///
	/// If `huge_memory_page_size` is `Some(None)`, then it uses the default huge page size (a boot time option for the kernel).
	/// `prefault` causes the page tables for a mapping to be loaded; for a file, this causes read-ahead; this is a performance tuning knob.
	/// `reserve_swap_space`, if false, ensures that overcommit is not used for this mapping and running out of memory on rite causes a segmentation fault (`SIGSEGV` signal).
	///
	/// If the defaults indicate `huge_memory_page_size` `Some(Some(huge_page_size))` is not supported, they will try to find a smaller supported huge page size; if there are not supported huge pages, then the mapping will not use huge pages.
	/// If the defaults indicate `huge_memory_page_size` `Some(None)` is not supported, they then the mapping will not use huge pages.
	///
	/// Linux's `MAP_LOCKED` and `MAP_GROWSDOWN` are not supported.
	/// `MAP_LOCKED` is a more 'extreme' variant of `prefault` that does not have as strong a g'tee as `mlock()`.
	///
	/// If using a file descriptor created using `MemoryFileDescriptor::open_anonymous_memory_as_file()`, then the value of `huge_memory_page_size` must be the same as used in the call to `MemoryFileDescriptor::open_anonymous_memory_as_file()`.
	///
	/// `length` will be rounded up to page size.
	/// `address_hint`'s `virtual_address_preference` will be rounded up to page size.
	///
	/// If `address_hint`, on x86_64, has `constrain_to_first_2Gb` as `true`, and the resultant `address + length` once rounded would exceed 2Gb, then a panic occurs.
	#[inline(always)]
	pub fn anonymous(length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, prefault: bool, reserve_swap_space: bool, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<Self, CreationError>
	{
		Self::new::<File>(None, length, address_hint, protection, sharing, prefault, reserve_swap_space, page_size_or_huge_page_size_settings)
	}
	
	/// As for `anonymous()`, but `offset` will be rounded up to page size.
	/// If `rounded_up(offset) + rounded_up(length)` exceeds the length of the underlying file, then the resultant memory after the end of the file will be filled with `0x00`.
	#[inline(always)]
	pub fn from_file<F: MemoryMappableFileDescriptor>(file_descriptor: &F, offset: u64, length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, prefault: bool, reserve_swap_space: bool, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<Self, CreationError>
	{
		Self::new(Some((file_descriptor, offset)), length, address_hint, protection, sharing, prefault, reserve_swap_space, page_size_or_huge_page_size_settings)
	}
	
	/// Returns `Ok(true)` if memory was locked.
	/// Returns `Ok(false)` if only some (or none) of memory was locked but locking can be retried.
	#[inline(always)]
	pub fn lock(&self, memory_lock_settings: MemoryLockSettings) -> io::Result<bool>
	{
		self.lock_range(memory_lock_settings, 0..self.size)
	}
	
	/// Returns `Ok(true)` if memory was locked.
	/// Returns `Ok(false)` if only some (or none) of memory was locked but locking can be retried.
	///
	/// `range.start` must be a multiple of `PageSize::current()`.
	#[inline(always)]
	pub fn lock_range(&self, memory_lock_settings: MemoryLockSettings, range: Range<usize>) -> io::Result<bool>
	{
		debug_assert!(range.end <= self.size);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(range.start));
		
		let result = unsafe { mlock2(self.virtual_address.add(range.start).into(), range.end - range.start, memory_lock_settings as u32) };
		if likely!(result == 0)
		{
			Ok(true)
		} else if likely!(result == -1)
		{
			match errno().0
			{
				EAGAIN => Ok(false),
				
				ENOMEM => panic!("the caller had a nonzero RLIMIT_MEMLOCK soft resource limit, but tried to lock more memory than the limit permitted. This limit is not enforced if the process is privileged (CAP_IPC_LOCK). Or, Some of the specified address range does not correspond to mapped pages in the address space of the process. Or, Locking or unlocking a region would result in the total number of mappings with distinct attributes (eg, locked versus unlocked) exceeding the allowed maximum.  (For example, unlocking a range in the middle of a currently locked mapping would result in three mappings: two locked mappings at each end and an unlocked mapping in the middle)"),
				EPERM => panic!("The caller is not privileged, but needs privilege (CAP_IPC_LOCK) to perform the requested operation."),
				EINVAL => panic!("The result of the addition addr+len was less than addr (eg, the addition may have resulted in an overflow). Or, Unknown flags were specified"),
				
				unexpected @ _ => panic!("Unexpected error {} from mlock2()", unexpected)
			}
		} else {
			unreachable_code(format_args!("Unexpected result {} from mlock2()", result))
		}
	}
	
	/// Returns `Ok(true)` if memory was unlocked.
	/// Returns `Ok(false)` if only some (or none) of memory was unlocked but unlocking can be retried.
	#[inline(always)]
	pub fn unlock(&self) -> io::Result<bool>
	{
		self.unlock_range(0..self.size)
	}
	
	/// Returns `Ok(true)` if memory was unlocked.
	/// Returns `Ok(false)` if only some (or none) of memory was unlocked but unlocking can be retried.
	///
	/// `range.start` must be a multiple of `PageSize::current()`.
	#[inline(always)]
	pub fn unlock_range(&self, range: Range<usize>) -> io::Result<bool>
	{
		debug_assert!(range.end <= self.size);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(range.start));
		
		let result = unsafe { munlock(self.virtual_address.add(range.start).into(), range.end - range.start) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EAGAIN => Ok(false),
				
				ENOMEM => panic!("the caller had a nonzero RLIMIT_MEMLOCK soft resource limit, but tried to lock more memory than the limit permitted. This limit is not enforced if the process is privileged (CAP_IPC_LOCK). Or, Some of the specified address range does not correspond to mapped pages in the address space of the process. Or, Locking or unlocking a region would result in the total number of mappings with distinct attributes (eg, locked versus unlocked) exceeding the allowed maximum.  (For example, unlocking a range in the middle of a currently locked mapping would result in three mappings: two locked mappings at each end and an unlocked mapping in the middle)"),
				EPERM => panic!("The caller is not privileged, but needs privilege (CAP_IPC_LOCK) to perform the requested operation."),
				EINVAL => panic!("The result of the addition addr+len was less than addr (eg, the addition may have resulted in an overflow)."),
				
				unexpected @ _ => panic!("Unexpected error {} from munlock()", unexpected)
			}
		} else {
			unreachable_code(format_args!("Unexpected result {} from mlock2()", result))
		}
	}
	
	/// Advise Linux kernel of usage of this memory.
	///
	/// If the Linux kernel wasn't compiled with `CONFIG_ADVISE_SYSCALLS`, this system call will fail.
	#[inline(always)]
	pub fn advise(&self, advice: MemoryAdvice) -> io::Result<()>
	{
		self.advise_range(advice, 0..self.size)
	}
	
	/// Advise Linux kernel of usage of this memory.
	///
	/// If the Linux kernel wasn't compiled with `CONFIG_ADVISE_SYSCALLS`, this system call will fail.
	///
	/// `range.start` must be a multiple of `PageSize::current()`.
	#[inline(always)]
	pub fn advise_range(&self, advice: MemoryAdvice, range: Range<usize>) -> io::Result<()>
	{
		debug_assert!(range.end <= self.size);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(range.start));
		
		let result = unsafe { madvise(self.virtual_address.add(range.start).into(), range.end - range.start, advice as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("madvise() returned unexpected result {}", result))
		}
	}
	
	/// Does not support the obsolete `PROT_SEM` flag.
	/// Does not support combining the PowerPC-only `PROT_SAO` flag with other flags to minimize syscalls, sorry.
	/// Does not support combining the deprecated `PROT_GROWSUP` and `PROT_GROWSDOWN` flags with other flags to minimize syscalls, sorry.
	#[inline(always)]
	pub fn change_protection(&self, protection: ExtendedProtection) -> io::Result<()>
	{
		self.change_protection_range(protection, 0..self.size)
	}
	
	/// Does not support the obsolete `PROT_SEM` flag.
	/// Does not support combining the PowerPC-only `PROT_SAO` flag with other flags to minimize syscalls, sorry.
	/// Does not support combining the deprecated `PROT_GROWSUP` and `PROT_GROWSDOWN` flags with other flags to minimize syscalls, sorry.
	///
	/// `range.start` must be a multiple of `PageSize::current()`.
	#[inline(always)]
	pub fn change_protection_range(&self, protection: ExtendedProtection, range: Range<usize>) -> io::Result<()>
	{
		debug_assert!(range.end <= self.size);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(range.start));
		
		let result = unsafe { mprotect(self.virtual_address.add(range.start).into(), range.end - range.start, protection as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("mprotect() returned unexpected result {}", result))
		}
	}
	
	/// Synchronize a file-backed mapping.
	///
	/// Returns `Err()` if `synchronize` asked to invalidate and a memory lock exists which covers all or part of `range`.
	#[inline(always)]
	pub fn synchronize_with_backing_file(&self, synchronize: SynchronizeFlags) -> Result<(), ()>
	{
		self.synchronize_with_backing_file_range(synchronize, 0..self.size)
	}
	
	/// Synchronize a file-backed mapping.
	///
	/// Returns `Err()` if `synchronize` asked to invalidate and a memory lock exists which covers all or part of `range`.
	///
	/// `range.start` must be a multiple of `PageSize::current()`.
	#[inline(always)]
	pub fn synchronize_with_backing_file_range(&self, synchronize: SynchronizeFlags, range: Range<usize>) -> Result<(), ()>
	{
		self.guard_range(&range);
		
		let result = unsafe { msync(self.virtual_address.add(range.start).into(), range.end - range.start, synchronize as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::SynchronizeFlags::*;
			match errno().0
			{
				EBUSY => match synchronize
				{
					AsynchronousAndInvalidate | SynchronousAndInvalidate => Err(()),
					Asynchronous | Synchronous => panic!("Unexpected error EBUSY from msync()"),
				},
				
				EINVAL => panic!("addr is not a multiple of PAGESIZE; or any bit other than MS_ASYNC | MS_INVALIDATE | MS_SYNC is set in flags; or both MS_SYNC and MS_ASYNC are set in flags."),
				ENOMEM => panic!("The indicated memory (or part of it) was not mapped."),
				
				unexpected @ _ => panic!("Unexpected error {} from msync()", unexpected),
			}
		}
		else
		{
			unreachable_code(format_args!("mprotect() returned unexpected result {}", result))
		}
	}
	
	/// Remap memory.
	///
	/// `new_size` is rounded up to the mapping's page size.
	///
	/// If `new_size` is larger than the existing size, the new pages are mapped in already zeroed.
	#[inline(always)]
	pub fn remap(&mut self, new_size: NonZeroU64, hints: RemapMemoryHints) -> Result<(), ()>
	{
		let old_size = self.size;
		let new_size = self.page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(new_size.get()) as usize;
		
		let (to_address, flags, new_virtual_address) = hints.to_address_and_flags(self.page_size, self.virtual_address);
		
		let result = unsafe { mremap(self.virtual_address.into(), old_size, new_size, flags, to_address) };
		if unlikely!(result == MAP_FAILED)
		{
			return Err(())
		}
		
		self.size = new_size;
		self.virtual_address = new_virtual_address;
		
		Ok(())
	}
	
	/// Owns this reference.
	#[inline(always)]
	pub fn owns_reference<E>(&self, reference: &E) -> bool
	{
		self.owns_non_null(new_non_null(reference as *const E as *mut E as *mut u8))
	}
	
	/// Owns this non-null.
	#[inline(always)]
	pub fn owns_non_null<E>(&self, non_null: NonNull<E>) -> bool
	{
		self.owns_pointer(non_null.as_ptr() as *const E)
	}
	
	/// Owns this pointer.
	///
	/// Treats memory as a `Range`, so does not own the pointer if it is equal to `self.virtual_address() + self.mapped_size_in_bytes()`.
	#[inline(always)]
	pub fn owns_pointer<E>(&self, pointer: *const E) -> bool
	{
		pointer as *const u8 as usize;
		let start: *const E = self.virtual_address.into();
		if unlikely!(start > pointer)
		{
			return false
		}
		let end = self.virtual_address.offset_in_bytes(self.size).into();
		pointer < end
	}
	
	/// Virtual address.
	#[inline(always)]
	pub fn virtual_address(&self) -> VirtualAddress
	{
		self.virtual_address
	}
	
	/// Mapped page size used.
	#[inline(always)]
	pub fn page_size(&self) -> PageSizeOrHugePageSize
	{
		self.page_size
	}
	
	/// Mapped size in bytes.
	#[inline(always)]
	pub fn mapped_size_in_bytes(&self) -> usize
	{
		self.size
	}
	
	/// Mapped size in number of pages.
	#[inline(always)]
	pub fn number_of_pages(&self) -> usize
	{
		self.size / (self.page_size_in_bytes().get() as usize)
	}
	
	/// Zeros all mapped memory.
	#[inline(always)]
	pub fn zero(&self)
	{
		let pointer: *mut u8 = self.virtual_address.into();
		unsafe { pointer.write_bytes(0x00, self.size) }
	}
	
	/// Removes page of `self.page_size` (which might be huge pages) from the end of this mapping.
	#[inline(always)]
	pub fn remove_from_end(&mut self, pages_to_remove: NonZeroNumberOfPages)
	{
		let length_to_remove = self.length_to_remove(pages_to_remove);
		
		self.unmap(length_to_remove)
	}
	
	/// Removes page of `self.page_size` (which might be huge pages) from the front of this mapping.
	#[inline(always)]
	pub fn remove_from_front(&mut self, pages_to_remove: NonZeroNumberOfPages)
	{
		let length_to_remove = self.length_to_remove(pages_to_remove);
		
		drop
		(
			Self
			{
				virtual_address: self.virtual_address,
				size: length_to_remove,
				page_size: self.page_size,
			}
		);
		self.virtual_address = self.virtual_address.offset_in_bytes(length_to_remove);
		self.size -= length_to_remove;
	}
	
	#[inline(always)]
	fn new<F: MemoryMappableFileDescriptor>(anonymous_or_file_descriptor: Option<(&F, u64)>, length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, prefault: bool, reserve_swap_space: bool, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<Self, CreationError>
	{
		let (huge_page_size_flags, page_size_or_huge_page_size) = page_size_or_huge_page_size_settings.mmap_flag_bits_and_page_size();
		
		let length_in_bytes = page_size_or_huge_page_size.non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(length).get();
		
		let (address, address_flags) = address_hint.to_address_and_flags(page_size_or_huge_page_size, length_in_bytes);
		
		let (file_descriptor, anonymous_flags, offset_in_bytes) = match anonymous_or_file_descriptor
		{
			None => (-1, MAP_ANONYMOUS, 0),
			Some((file, offset_in_bytes)) => (file.as_raw_fd(), 0, page_size_or_huge_page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(offset_in_bytes)),
		};
		
		let prefault_flags = if prefault
		{
			MAP_POPULATE
		}
		else
		{
			0
		};
		
		let no_reserve_flags = if reserve_swap_space
		{
			0
		}
		else
		{
			MAP_NORESERVE
		};
		
		let flags = address_flags | anonymous_flags | sharing as i32 | huge_page_size_flags | prefault_flags | no_reserve_flags;
		
		let result = unsafe { mmap(address, length_in_bytes.try_into().unwrap(), protection as i32, flags, file_descriptor, offset_in_bytes.try_into().unwrap()) };
		if unlikely!(result == MAP_FAILED)
		{
			use self::CreationError::*;
			
			match errno().0
			{
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
				EPERM => Err(PermissionDenied),
				
				EEXIST => panic!("MAP_FIXED_NOREPLACE was specified in flags, and the range covered by addr and length clashes with an existing mapping."),
				ENODEV => panic!("The underlying filesystem of the specified file does not support memory mapping"),
				EACCES => panic!("A file descriptor refers to a non-regular file. Or a file mapping was requested, but fd is not open for reading. Or MAP_SHARED was requested and PROT_WRITE is set, but fd is not open in read/write (O_RDWR) mode. Or PROT_WRITE is set, but the file is append-only"),
				
				EAGAIN => panic!("The file has been locked, or too much memory has been locked (see man 2 setrlimit"),
				EBADF => panic!("fd is not a valid file descriptor (and MAP_ANONYMOUS was not set)"),
				EINVAL => panic!("We don't like addr, length, or offset (e.g., they are too large, or not aligned on a page boundary). Or, flags contained none of MAP_PRIVATE, MAP_SHARED or MAP_SHARED_VALIDATE. Or, since Linux 2.6.12, length was 0"),
				EOVERFLOW => panic!("This should only occur on 32-bit architectures"),
				ETXTBSY => panic!("Legacy ETXTBUSY error; MAP_DENYWRITE was set but the object specified by fd is open for writing"),
				
				unexpected @ _ => panic!("Unexpected error `{}`", unexpected),
			}
		}
		else
		{
			Ok
			(
				Self
				{
					virtual_address: VirtualAddress::from(result),
					size: length_in_bytes as usize,
					page_size: page_size_or_huge_page_size,
				},
			)
		}
	}
	
	/// Get value.
	///
	/// `T: Copy` to force `T` to not implement `Drop` as otherwise a memory leak could occur.
	#[inline(always)]
	pub fn get_volatile<T: Copy>(&self, offset: usize) -> T
	{
		let pointer = self.pointer_to(offset);
		unsafe { read_volatile(pointer) }
	}
	
	/// Set value.
	///
	/// `T: Copy` to force `T` to not implement `Drop` as otherwise a memory leak could occur.
	#[inline(always)]
	pub fn set_volatile<T: Copy>(&self, offset: usize, value: T)
	{
		let pointer = self.mut_pointer_to(offset);
		unsafe { write_volatile(pointer, value) }
	}
	
	/// Returns the chosen size that best fits huge pages and the chosen huge page setting to use.
	///
	/// Returns `None` if `preferred_buffer_size` exceeds `2^63`.
	#[inline(always)]
	pub fn size_suitable_for_a_power_of_two_ring_queue(preferred_buffer_size: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes, inclusive_maximum_bytes_wasted: u64) -> Option<(u64, PageSizeOrHugePageSizeSettings)>
	{
		let buffer_size_power_of_two_at_least_one_page = match Self::round_buffer_size_up_to_power_of_two(preferred_buffer_size)
		{
			None => return None,
			Some(value) => Self::round_buffer_size_up_to_smallest_page_size(value)
		};
		
		let (buffer_size, huge_page_size) = match defaults.best_fit_huge_page_size_if_any(buffer_size_power_of_two_at_least_one_page, inclusive_maximum_bytes_wasted)
		{
			None => (buffer_size_power_of_two_at_least_one_page, PageSizeOrHugePageSizeSettings::for_page_size(defaults)),
			
			Some(huge_page_size) => (huge_page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(buffer_size_power_of_two_at_least_one_page), PageSizeOrHugePageSizeSettings::for_huge_page_size(huge_page_size))
		};
		
		Some((buffer_size, huge_page_size))
	}

	#[inline(always)]
	fn round_buffer_size_up_to_power_of_two(preferred_buffer_size: NonZeroU64) -> Option<u64>
	{
		preferred_buffer_size.get().checked_next_power_of_two()
	}
	
	#[inline(always)]
	fn round_buffer_size_up_to_smallest_page_size(buffer_size_power_of_two: u64) -> u64
	{
		PageSize::current().number_of_bytes_rounded_up_to_multiple_of_page_size(buffer_size_power_of_two)
	}

	/// Pointer to value.
	#[inline(always)]
	fn pointer_to<T>(&self, offset: usize) -> *const T
	{
		self.mut_pointer_to(offset) as *const T
	}

	/// Pointer to value.
	#[inline(always)]
	fn mut_pointer_to<T>(&self, offset: usize) -> *mut T
	{
		self.virtual_address.pointer_to(offset).as_ptr()
	}

	#[inline(always)]
	fn page_size_in_bytes(&self) -> NonZeroU64
	{
		self.page_size.size_in_bytes()
	}

	#[inline(always)]
	fn length_to_remove(&self, pages_to_remove: NonZeroNumberOfPages) -> usize
	{
		let length_to_remove = (pages_to_remove.get() * self.page_size_in_bytes().get()) as usize;

		debug_assert!(length_to_remove <= self.size, "length_to_remove {} exceeds size {}", length_to_remove, self.size);

		length_to_remove
	}

	#[inline(always)]
	fn unmap(&mut self, length_to_remove_from_end: usize)
	{
		debug_assert_ne!(length_to_remove_from_end, 0, "munmap() is not allowed to have a length_to_remove_from_end from of zero");

		let new_size = self.size - length_to_remove_from_end;

		let result = unsafe { munmap(self.virtual_address.offset_in_bytes(new_size).into(), length_to_remove_from_end as usize) };
		if likely!(result == 0)
		{
			self.size = new_size;
			return
		}
		else if likely!(result == -1)
		{
			// `ENOMEM` can occur if a memory mapping is 'split' with the central section unmapped.
			panic!("munmap() returned an error of {}", errno())
		}
		else
		{
			panic!("munmap() failed with an unexpected exit code of {:?}", result)
		}
	}

	#[inline(always)]
	pub(crate) fn guard_range(&self, range: &Range<usize>)
	{
		debug_assert!(range.end <= self.size);
		debug_assert!(PageSize::is_an_exact_page_size_multiple_of_current_usize(range.start))
	}
}
