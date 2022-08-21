// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents an event instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventFileDescriptor(RawFd);

impl Drop for EventFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for EventFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for EventFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for EventFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for EventFileDescriptor
{
}

impl EventFileDescriptor
{
	/// Creates a new instance.
	///
	/// The `initial_value` can not be `u64::MAX`.
	#[inline(always)]
	pub fn new(initial_value: u64, use_as_a_semaphore: bool) -> Result<Self, CreationError>
	{
		debug_assert_ne!(initial_value, u64::MAX, "initial_value can not be u64::MAX");

		const CommonFlags: c_int = EFD_NONBLOCK | EFD_CLOEXEC;

		let flags = if use_as_a_semaphore
		{
			CommonFlags | EFD_SEMAPHORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { eventfd(initial_value, flags) };
		if likely!(result != -1)
		{
			Ok(EventFileDescriptor(result))
		}
		else
		{
			use self::CreationError::*;

			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid arguments"),
					ENODEV => panic!("Could not mount (internal) anonymous inode device"),
					unexpected_error @ _ => unexpected_error!(eventfd, unexpected_error),
				}
			)
		}
	}

	/// Reads an event.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	///
	/// The Ok() result is either:-
	///
	/// * The value of the event counter, or,
	/// * If this is a semaphore, the value 1.
	///
	/// After a Ok() read, the event counter is reset:-
	///
	/// * If this is not a semaphore, it is set to 0;
	/// * If this is a semaphore, it is decremented by 1.
	#[inline(always)]
	pub fn read(&self) -> Result<u64, StructReadError>
	{
		let mut value: u64 = unsafe_uninitialized();

		const SizeOfRead: usize = size_of::<u64>();

		let result = unsafe { libc::read(self.as_raw_fd(), &mut value as *mut _ as *mut _, SizeOfRead) };

		if likely!(result == SizeOfRead as isize)
		{
			Ok(value)
		}
		else
		{
			match result
			{
				-1 =>
				{
					use self::StructReadError::*;

					match SystemCallErrorNumber::from_errno_panic()
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						EISDIR => panic!("`fd` refers to a directory"),

						unexpected_error @ _ => unexpected_error!(read, "event file descriptor", unexpected_error),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				unexpected @ _ => unexpected_result!(read, "event file descriptor", unexpected),
			}
		}
	}

	/// Write an event increment.
	#[inline(always)]
	pub fn write(&self, increment: &u64) -> Result<(), StructWriteError>
	{
		debug_assert_ne!(*increment, u64::MAX, "increment may not be u64::MAX");

		use self::StructWriteError::*;

		const SizeOfWrite: usize = size_of::<u64>();

		let result = unsafe { libc::write(self.0, increment as *const _ as *const _, SizeOfWrite) };

		if likely!(result == SizeOfWrite as isize)
		{
			Ok(())
		}
		else
		{
			match result
			{
				-1 =>
				{
					match SystemCallErrorNumber::from_errno_panic()
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO | EPIPE => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						ENOSPC => panic!("out of space"),
						EDQUOT => panic!("out of quota"),
						EDESTADDRREQ => panic!("EDESTADDRREQ!"),
						EFBIG => panic!("EFBIG!"),
						
						unexpected_error @ _ => unexpected_error!(write, "event file descriptor", unexpected_error),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				unexpected @ _ => unexpected_result!(write, "event file descriptor", unexpected),
			}
		}
	}
}
