// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Represents an user fault instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserFaultFileDescriptor(RawFd);

impl Drop for UserFaultFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for UserFaultFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for UserFaultFileDescriptor
{
}

impl Into<Arc<Self>> for UserFaultFileDescriptor
{
	#[inline(always)]
	fn into(self) -> Arc<Self>
	{
		Arc::new(self)
	}
}

impl Into<PollingUserFaultFileDescriptor> for UserFaultFileDescriptor
{
	#[inline(always)]
	fn into(self) -> PollingUserFaultFileDescriptor
	{
		PollingUserFaultFileDescriptor(self.into())
	}
}

impl UserFaultFileDescriptor
{
	/// Creates a new non-blocking instance which is closed-on-exec.
	///
	/// `user_mode_only` should normally be `true`.
	/// `requested_features` should normally not contain `UFFD_FEATURE_EVENT_FORK` unless the caller has the `CAP_SYS_PTRACE` capability (will result in `EFAULT` to the api ioctl).
	#[inline(always)]
	pub fn new(user_mode_only: bool, requested_features: Features) -> Result<(Self, Features, Ioctls), CreationError>
	{
		let this = Self::create(user_mode_only)?;
		let (features, ioctls) = this.initialize(requested_features)?;
		Ok((this, features, ioctls))
	}
	
	#[inline(always)]
	fn create(user_mode_only: bool) -> Result<Self, CreationError>
	{
		const AlwaysOnFlags: i32 = O_CLOEXEC | O_NONBLOCK;
		let flags = if likely!(user_mode_only)
		{
			AlwaysOnFlags | UFFD_USER_MODE_ONLY
		}
		else
		{
			AlwaysOnFlags
		};
		let result = userfaultfd(flags);
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;
			
			match errno().0
			{
				EINVAL => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
				
				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
				
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				
				EPERM => Err(PermissionDenied),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}
	
	/// Obtains API details.
	///
	/// Features is always `UFFD_API_FEATURES`.
	/// Ioctls is always `UFFD_API_IOCTLS`.
	#[inline(always)]
	fn initialize(&self, requested_features: Features) -> Result<(Features, Ioctls), CreationError>
	{
		let mut api = uffdio_api::new(requested_features);
		match self.make_ioctl(UFFDIO_API, &mut api)
		{
			Ok(()) => Ok((api.features, api.ioctls)),
			
			Err(errno) => match errno.0
			{
				EPERM => Err(CreationError::PermissionDenied),
				
				EINVAL => panic!("Already initialized or bad arguments to ioctl"),
				
				EFAULT => panic!("argp does not point to a valid memory address"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_API)", errno),
			}
		}
	}
	
	/// Register memory range that has previously been mapped with `mmap()`.
	///
	/// Returns the `Ioctl` operations permitted on the memory, which always fall into one of three sets which can be known without examing the return type:-
	///
	/// * Those suitable for memory backed by huge pages: always `Ioctls::HugePages`.
	/// * Those suitable for memory backed by regular pages: always `Ioctls::RegularPages`.
	/// * Those suitable for memory backed by regular pages and `register_mode` including `RegisterMode::AllowWriteProtectedCopying`: always `Ioctls::RegularPagesWithWriteProtectOnCopy`.
	///
	/// Returns errors:-
	///
	/// * `KernelWouldBeOutOfMemory` if `ENOMEM` occurs (this may be because the memory has become unmapped).
	/// * `PermissionDenied` if `EPERM` occurs; this is typically because the mapped memory is `MAP_SHARED` and the process does not have write permissions to the underlying file (this includes checking file seals); writes can occur even for read-only files because of the way `copy()` is allowed to copy into sparse file holes.
	///
	/// `mapped_memory`:-
	///
	/// * must be page-aligned to memory mapped using `mmap()`; if using huge pages, then they must be aligned to the huge page alignment of that memory.
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// Note that if the mapped memory is using huge-pages then only 'basic' `Ioctls` are allowed.
	#[inline(always)]
	pub fn register_memory_range(&self, mapped_absolute_memory_range: impl AbsoluteMemoryRange, register_mode: RegisterMode) -> Result<Ioctls, CreationError>
	{
		use self::CreationError::*;
		
		let mut register = uffdio_register
		{
			range: Self::to_uffdio_range(mapped_absolute_memory_range),
			mode: register_mode,
			ioctls: Ioctls::empty()
		};
		match self.make_ioctl(UFFDIO_REGISTER, &mut register)
		{
			Ok(()) => Ok(register.ioctls),
			
			Err(errno) => match errno.0
			{
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				
				EPERM => Err(PermissionDenied),
				
				EBUSY => panic!("A mapping in the specified range is registered with another userfaultfd object"),
				
				EFAULT => panic!("argp does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or an invalid or unsupported bit was specified in the mode field; or the mode field was zero; or there is no mapping in the specified address range; or range.start or range.len is not a multiple of the system page size (or huge page size); or, range.len is zero; or these fields are otherwise invalid; or there as an incompatible mapping in the specified address range, ie one that does not support userfaults"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_REGISTER)", errno),
			}
		}
	}
	
	/// Unregister memory range that has previously been registered with `self.register_memory_range()`.
	///
	/// `mapped_memory`:-
	///
	/// * must be page-aligned to memory mapped using `mmap()`; if using huge pages, then they must be aligned to the huge page alignment of that memory.
	/// * can be a sub range of memory mapped using `mmap()`.
	///
	/// Returns errors:-
	///
	/// * `KernelWouldBeOutOfMemory` if `ENOMEM` occurs (this may be because the memory has become unmapped).
	#[inline(always)]
	pub fn unregister_memory_range(&self, mapped_absolute_memory_range: impl AbsoluteMemoryRange) -> Result<(), CreationError>
	{
		let mut range = Self::to_uffdio_range(mapped_absolute_memory_range);
		match self.make_ioctl(UFFDIO_UNREGISTER, &mut range)
		{
			Ok(()) => Ok(()),
			
			Err(errno) => match errno.0
			{
				EFAULT => panic!("argp does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or either the start or the len field of the ufdio_range structure was not a multiple of the system page size; or the len field was zero; or these fields were otherwise invalid; or, there as an incompatible mapping in the specified address range; or, there was no mapping in the specified address range"),
				
				ENOMEM => Err(CreationError::KernelWouldBeOutOfMemory),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_UNREGISTER)", errno),
			}
		}
	}
	
	/// Called from polling thread.
	///
	/// Ideally should be done after `poll()` or `epoll()`.
	/// Errors such as `POLLERR` are apparently possibly.
	#[inline(always)]
	pub fn read_events<'a>(&self, events: &'a mut [uffd_msg]) -> usize
	{
		const MessageSize: usize = size_of::<uffd_msg>();
		
		let maximum_number_of_messages_to_read = events.len();
		let mut buf = events as *mut uffd_msg as *mut c_void;
		let result = unsafe { libc::read(self.0, buf, MessageSize * maximum_number_of_messages_to_read) };
		if likely!(result >= 0)
		{
			let number_of_messages = (result as usize) / MessageSize;
			debug_assert!(number_of_messages <= maximum_number_of_messages_to_read);
			
			number_of_messages
		}
		else if likely!(result == -1)
		{
			panic!("userfaultfd read failed with `{}`", errno());
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}
	
	/// Called from polling thread; loops until all bytes copied.
	///
	/// `to_mapped_absolute_memory_range` and `from` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range (with the page size being of the same huge page size if appropriate).
	///
	/// `to_mapped_absolute_memory_range`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn copy(&self, to_mapped_absolute_memory_range: impl AbsoluteMemoryRange, copy_mode: CopyMode, from: VirtualAddress) -> Result<(), &'static str>
	{
		let (dst, len) = to_mapped_absolute_memory_range.inclusive_absolute_start_and_length_u64();
		
		let mut copy = uffdio_copy
		{
			dst,
			src: from.into(),
			len,
			mode: copy_mode,
			copy: 0,
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_COPY, &mut copy)
			{
				Ok(()) => return Ok(()),
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.copy as u64;
						
						copy.dst += bytes_copied;
						copy.src += bytes_copied;
						copy.len -= bytes_copied;
					}
					
					EFAULT => panic!("argp does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process or invalid copy mode"),
					
					ENOENT => return Err("The faulting process has changed its virtual memory layout simultaneously with an outstanding UFFDIO_COPY operation"),
					
					ESRCH => return Err("The faulting process has exited at the time of a UFFDIO_COPY operation"),
					
					ENOSPC => panic!("`ENOSPC` was only returned for Linux version 4.11 until Linux version 4.13"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_COPY)", errno),
				}
			}
		}
	}
	
	/// Called from polling thread; loops until all bytes copied.
	///
	/// Writes zeroes to page(s) (logically, copies from pages of zeroes).
	///
	/// __NOTE: This API is not supported by Linux for registered memory using huge pages__.
	///
	/// `to_mapped_absolute_memory_range` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	///
	/// `to_mapped_absolute_memory_range`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn copy_zero_page(&self, to_mapped_absolute_memory_range: impl AbsoluteMemoryRange, zero_page_mode: ZeroPageMode) -> Result<(), &'static str>
	{
		let mut copy = uffdio_zeropage
		{
			range: Self::to_uffdio_range(to_mapped_absolute_memory_range),
			mode: zero_page_mode,
			zeropage: 0
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_ZEROPAGE, &mut copy)
			{
				Ok(()) => return Ok(()),
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.zeropage as u64;
						
						copy.range.start += bytes_copied;
						copy.range.len -= bytes_copied;
					}
					
					EFAULT => panic!("argp does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process or invalid zero page mode"),
					
					ENOENT => return Err("The faulting process has changed its virtual memory layout simultaneously with an outstanding UFFDIO_ZEROPAGE operation"),
					
					ESRCH => return Err("The faulting process has exited at the time of a UFFDIO_ZEROPAGE operation"),
					
					// Not officially documented but may have been possible.
					ENOSPC => panic!("`ENOSPC` was only returned for Linux version 4.11 until Linux version 4.13"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_ZEROPAGE)", errno),
				}
			}
		}
	}
	
	/// Called from polling thread.
	///
	/// Wake up the thread waiting for page fault resolution.
	///
	/// Used after calling either `copy()` or `copy_zero_page()` or both once or more with a mode of `DoNotWakeUp`.
	///
	/// `to_mapped_absolute_memory_range` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	///
	/// `to_mapped_absolute_memory_range`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn wake_up_after_copy(&self, to_mapped_absolute_memory_range: impl AbsoluteMemoryRange)
	{
		let mut wake_up = Self::to_uffdio_range(to_mapped_absolute_memory_range);
		
		match self.make_ioctl(UFFDIO_WAKE, &mut wake_up)
		{
			Ok(()) => (),
			
			Err(errno) => match errno.0
			{
				EFAULT => panic!("argp does not point to a valid memory address"),
				
				EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or memory addresses or length not aligned to page size or are outside of permitted range for process"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_WAKE)", errno),
			}
		}
	}
	
	/// Called from polling thread; loops until complete.
	///
	/// `to_mapped_absolute_memory_range` must be:-
	///
	/// * page-aligned.
	/// * encapsulated within a registered memory range.
	/// * have been __registered__ (`self.register_mapped_memory()`) with `register_mode` containing `RegiserMode::WriteProtect`.
	///
	/// `to_mapped_absolute_memory_range`:-
	///
	/// * can be a sub range of memory mapped using `mmap()`.
	#[inline(always)]
	pub fn wake_up_after_copy_write_protect(&self, to_mapped_absolute_memory_range: impl AbsoluteMemoryRange, write_protect_mode: WriteProtectMode)
	{
		let mut write_protect = uffdio_writeprotect
		{
			range: Self::to_uffdio_range(to_mapped_absolute_memory_range),
			
			mode: write_protect_mode,
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_UNREGISTER, &mut write_protect)
			{
				Ok(()) => return,
				
				Err(errno) => match errno.0
				{
					EAGAIN => continue,
					
					EFAULT => panic!("argp does not point to a valid memory address"),
					
					EINVAL => panic!("The userfaultfd object has not yet been enabled (via the UFFDIO_API operation); or either the start or the len field of the ufdio_range structure was not a multiple of the system page size; or the len field was zero; or these fields were otherwise invalid; or, there as an incompatible mapping in the specified address range; or, there was no mapping in the specified address range"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_UNREGISTER)", errno),
				}
			}
		}
	}
	
	#[inline(always)]
	fn to_uffdio_range(to_mapped_absolute_memory_range: impl AbsoluteMemoryRange) -> uffdio_range
	{
		let (start, len) = to_mapped_absolute_memory_range.inclusive_absolute_start_and_length_u64();
		
		uffdio_range
		{
			start,
			len,
		}
	}
	
	#[inline(always)]
	fn make_ioctl<V>(&self, request: u64, value: &mut V) -> Result<(), Errno>
	{
		let result = unsafe { ioctl(self.0, UFFDIO_API, value as *mut V as *mut c_void) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(errno())
		}
		else
		{
			panic!("Unexpected result {}", result);
		}
	}
}
