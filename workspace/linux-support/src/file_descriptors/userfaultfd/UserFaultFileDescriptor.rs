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
				
				EFAULT => panic!("Could not copy user memory"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_API)", errno),
			}
		}
	}
	
	/// Register memory range.
	#[inline(always)]
	pub fn register_memory_range(&self, start: VirtualAddress, length: u64, mode: RegisterMode) -> Ioctls
	{
		let mut register = uffdio_register::new(start, length, mode);
		match self.make_ioctl(UFFDIO_REGISTER, &mut register)
		{
			Ok(()) => register.ioctls,
			
			Err(errno) => match errno.0
			{
				EBUSY => panic!("A mapping in the specified range is registered with another userfaultfd object"),
				
				EFAULT => panic!("argp refers to an address that is outside the calling process's accessible address space"),
				
				EINVAL => panic!("An invalid or unsupported bit was specified in the mode field; or the mode field was zero; or there is no mapping in the specified address range; or range.start or range.len is not a multiple of the system page size; or, range.len is zero; or these fields are otherwise invalid; or there as an incompatible mapping in the specified address range"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_REGISTER)", errno),
			}
		}
	}
	
	/// Unregister memory range.
	#[inline(always)]
	pub fn unregister_memory_range(&self, start: VirtualAddress, length: u64)
	{
		let mut range = uffdio_range
		{
			start: start.into(),
		
			len: length
		};
		match self.make_ioctl(UFFDIO_UNREGISTER, &mut range)
		{
			Ok(()) => (),
			
			Err(errno) => match errno.0
			{
				EINVAL => panic!("Either the start or the len field of the ufdio_range structure was not a multiple of the system page size; or the len field was zero; or these fields were otherwise invalid; or, there as an incompatible mapping in the specified address range; or, there was no mapping in the specified address range"),
				
				_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_UNREGISTER)", errno),
			}
		}
	}
	
	/// Called from polling thread.
	///
	/// Ideally should be done after `poll()` or `epoll()`.
	#[inline(always)]
	pub fn read_events<'a>(&self, events: &'a mut [uffd_msg]) -> &'a [uffd_msg]
	{
		const MessageSize: usize = size_of::<uffd_msg>();
		
		let maximum_number_of_messages_to_read = events.len();
		let mut buf = events as *mut uffd_msg as *mut c_void;
		let result = unsafe { libc::read(self.0, buf, MessageSize * maximum_number_of_messages_to_read) };
		if likely!(result >= 0)
		{
			let number_of_messages = (result as usize) / MessageSize;
			debug_assert!(number_of_messages <= maximum_number_of_messages_to_read);
			
			&events[.. number_of_messages]
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
	
	/// Called from polling thread.
	///
	/// `from`, `to` and `length` must be page-aligned (with the page size being of the same huge page size if appropriate).
	#[inline(always)]
	pub fn copy(&self, from: VirtualAddress, to: VirtualAddress, length: u64, copy_mode: CopyMode) -> Result<(), &'static str>
	{
		let mut copy = uffdio_copy
		{
			dst: to.into(),
			src: from.into(),
			len: length,
			mode: copy_mode,
			copy: 0,
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_COPY, &mut copy)
			{
				Ok(()) =>
				{
					debug_assert_eq!(copy.copy as u64, length);
					return Ok(())
				}
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.copy as u64;
						
						debug_assert!(bytes_copied < length);
						// Not asserted, but `bytes_copied` should by page aligned.
						
						copy.dst += bytes_copied;
						copy.src += bytes_copied;
						copy.len -= bytes_copied;
					}
					
					EINVAL => panic!("Memory addresses or length not aligned to page size or are outside of permitted range for process or invalid copy mode"),
					
					EFAULT => panic!("Could not copy user memory"),
					
					ENOENT => return Err("The faulting process has changed its virtual memory layout simultaneously with an outstanding UFFDIO_COPY operation"),
					
					/// `ENOSPC` was only returned for Linux version 4.11 until Linux version 4.13.
					ENOSPC | ESRCH => return Err("The faulting process has exited at the time of a UFFDIO_COPY operation"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_COPY)", errno),
				}
			}
		}
	}
	
	/// Called from polling thread.
	///
	/// Writes zeroes to page(s) (logically, copies from pages of zeroes).
	///
	/// `to` and `length` must be page-aligned (with the page size being of the same huge page size if appropriate).
	#[inline(always)]
	pub fn zero_page(&self, to: VirtualAddress, length: u64, zero_page_mode: ZeroPageMode) -> Result<(), &'static str>
	{
		let mut copy = uffdio_zeropage
		{
			range: uffdio_range
			{
				start: to.into(),
				len: length,
			},
			mode: zero_page_mode,
			zeropage: 0
		};
		
		loop
		{
			match self.make_ioctl(UFFDIO_ZEROPAGE, &mut copy)
			{
				Ok(()) =>
				{
					debug_assert_eq!(copy.zeropage as u64, length);
					return Ok(())
				}
				
				Err(errno) => match errno.0
				{
					EAGAIN =>
					{
						let bytes_copied = copy.zeropage as u64;
						
						debug_assert!(bytes_copied < length);
						// Not asserted, but `bytes_copied` should by page aligned.
						
						copy.range.start += bytes_copied;
						copy.range.len -= bytes_copied;
					}
					
					EINVAL => panic!("Memory addresses or length not aligned to page size or are outside of permitted range for process or invalid zero page mode"),
					
					EFAULT => panic!("Could not copy user memory"),
					
					ENOENT => return Err("The faulting process has changed its virtual memory layout simultaneously with an outstanding UFFDIO_ZEROPAGE operation"),
					
					ESRCH => return Err("The faulting process has exited at the time of a UFFDIO_ZEROPAGE operation"),
					
					_ => panic!("Unexpect errno `{}` from userfaultfd ioctl(UFFDIO_ZEROPAGE)", errno),
				}
			}
		}
	}
	
	// TODO: UFFDIO_WAKE, UFFDIO_WRITEPROTECT?
	
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
