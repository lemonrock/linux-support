// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory mapped using `mmap()`.
#[derive(Debug)]
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
	pub fn anonymous(length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, huge_memory_page_size: Option<Option<HugePageSize>>, prefault: bool, reserve_swap_space: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		Self::new::<File>(None, length, address_hint, protection, sharing, huge_memory_page_size, prefault, reserve_swap_space, defaults)
	}

	/// As for `anonymous()`, but `offset` will be rounded up to page size.
	/// If `rounded_up(offset) + rounded_up(length)` exceeds the length of the underlying file, then the resultant memory after the end of the file will be filled with `0x00`.
	#[inline(always)]
	pub fn from_file<F: MemoryFileDescriptor>(file_descriptor: &F, offset: u64, length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, huge_memory_page_size: Option<Option<HugePageSize>>, prefault: bool, reserve_swap_space: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		Self::new(Some((file_descriptor, offset)), length, address_hint, protection, sharing, huge_memory_page_size, prefault, reserve_swap_space, defaults)
	}

	#[inline(always)]
	fn new<F: MemoryFileDescriptor>(anonymous_or_file_descriptor: Option<(&F, u64)>, length: NonZeroU64, address_hint: AddressHint, protection: Protection, sharing: Sharing, huge_page_size: Option<Option<HugePageSize>>, prefault: bool, reserve_swap_space: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		let (huge_page_size_flags, page_size) = HugePageSize::mmap_or_memfd_flag_bits_and_page_size(MAP_HUGETLB, huge_page_size, defaults);

		let length_in_bytes = page_size.non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(length).get();

		let (address, address_flags) = address_hint.to_address_and_flags(page_size, length_in_bytes);

		let (file_descriptor, anonymous_flags, offset_in_bytes) = match anonymous_or_file_descriptor
		{
			None => (-1, MAP_ANONYMOUS, 0),
			Some((memory_file_descriptor, offset_in_bytes)) => (memory_file_descriptor.as_raw_fd(), 0, page_size.number_of_bytes_rounded_up_to_multiple_of_page_size(offset_in_bytes)),
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
					page_size,
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
}