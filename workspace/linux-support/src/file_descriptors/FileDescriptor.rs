// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A marker trait to bring together all the properties of a file descriptor.
pub trait FileDescriptor: Sized + Debug + AsRawFd + FromRawFd + IntoRawFd
{
	/// Makes a copy that, when dropped, does not close the underlying file descriptor.
	///
	/// However, it is *not linked* to the lifetime of the underlying file descriptor and so may become invalid.
	///
	/// The `FileDescriptorCopy` can be used via `deref()` and `deref_mut()`.
	#[inline(always)]
	fn copy(&self) -> FileDescriptorCopy<Self>
	{
		FileDescriptorCopy::new(self.as_raw_fd())
	}
	
	/// Similar to `clone()` but underlying status (eg file seek position) is shared; file descriptor flags, on the other hand, are not.
	///
	/// Forces the new file descriptor to be close-on-exec.
	///
	/// Returns `None` if interupted by a signal or would be blocked.
	///
	/// NOTE: This does not use the `dup()`, `dup2()` or `dup3()` system calls but `fcntl()`.
	#[inline(always)]
	fn duplicate_with_close_on_exec_non_blocking(&self) -> Result<Option<Self>, CreationError>
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_DUPFD_CLOEXEC, 0) };
		if likely!(result >= 0)
		{
			Ok(Some(unsafe { Self::from_raw_fd(result) }))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			// Errors are a bit of a guess based on a reading of the manpages of `dup()` and `fcntl()`.
			match errno().0
			{
				EBUSY | EINTR | EAGAIN => Ok(None),

				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),

				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),

				ENOMEM | ENOSPC => Err(KernelWouldBeOutOfMemory),

				EBADF => panic!("oldfd isn't an open file descriptor"),

				unexpected @ _ => panic!("Unexpected error {} from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from fcntl()", result))
		}
	}

	/// Remove the close-on-exec flag (not recommended).
	#[inline(always)]
	fn remove_close_on_exec(&self)
	{
		let flags = self.get_file_descriptor_flags();
		let flags_without_close_on_exec = flags & !FD_CLOEXEC;

		let result = unsafe { fcntl (self.as_raw_fd(), F_SETFD, flags_without_close_on_exec) };
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			panic!("Error `{}` from fcntl()", errno())
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from fcntl()", result))
		}
	}

	/// Make a file descriptor blocking (all file descriptors in this library are ordinarily non-blocking)
	#[inline(always)]
	fn make_blocking(&self)
	{
		let flags = self.get_o_flags();
		let flags_without_non_block = flags & !O_NONBLOCK;

		let result = unsafe { fcntl(self.as_raw_fd(), F_SETFL, flags_without_non_block) };
		if likely!(result == 0)
		{
			()
		}
		else if likely!(result == -1)
		{
			panic!("Error from fcntl F_SETFL with `{}`", errno())
		}
		else
		{
			panic!("Unexpected result from fcntl F_SETFL of `{}`", result)
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_o_flags(&self) -> i32
	{
		Self::get_o_flags_raw_fd(self.as_raw_fd())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_o_flags_raw_fd(raw_fd: RawFd) -> i32
	{
		let result = unsafe { fcntl(raw_fd, F_GETFL) };
		if likely!(result >= 0)
		{
			result
		}
		else if likely!(result == -1)
		{
			panic!("Error from fcntl F_GETFL with `{}`", errno())
		}
		else
		{
			panic!("Unexpected result from fcntl F_GETFL of `{}`", result)
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	fn get_file_descriptor_flags(&self) -> i32
	{
		let file_descriptor_flags = unsafe { fcntl (self.as_raw_fd(), F_GETFD) };
		if unlikely!(file_descriptor_flags < 0)
		{
			panic!("Error getting file descriptor flags `{}`", errno())
		}
		file_descriptor_flags
	}
}
