// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Extensions for a pipe file descriptor to make it useful for cloned processes.
pub trait PipeFileDescriptor: FileDescriptor + OnDiskFileDescriptor + PipeLikeFileDescriptor
{
	/// Clones a pipe file descriptor so the pipe is accessible in a child process.
	fn clone_for_child_process(&self) -> Self;

	/// Changes the pipe capacity.
	///
	/// Processes with the capability `CAP_SYS_RESOURCE` may not set the capacity to greater than `maximum_pipe_capacity()`.
	///
	/// The value used may be rounded up by the Linux kernel (currently, the allocation is the next higher power-of-two page-size multiple of the requested size).
	///
	/// Returns an error if:-
	///
	/// * the caller tries to use a value greater than `maximum_pipe_capacity()` without having the capability `CAP_SYS_RESOURCE`;
	/// * the caller tries to set a capacity lower than the number of bytes currently in the pipe.
	fn change_capacity(&self, new_capacity: NonZeroU32) -> Result<NonZeroU32, ChangeCapacityError>
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_SETPIPE_SZ, new_capacity.get() as i32) };
		if likely!(result > 0)
		{
			Ok(new_non_zero_u32(result as u32))
		}
		else if likely!(result == -1)
		{
			use self::ChangeCapacityError::*;
			match SystemCallErrorNumber::from_errno_panic()
			{
				EPERM => Err(PermissionDenied),
				EBUSY => Err(WouldReduceCapacityBelowThatInUse),

				unexpected_error @ _ => unexpected_error!(fcntl, F_GETPIPE_SZ, unexpected_error)
			}
		}
		else
		{
			unexpected_result!(fcntl, F_GETPIPE_SZ, result)
		}
	}

	/// Get current capacity.
	fn get_capacity(&self) -> NonZeroU32
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_GETPIPE_SZ) };
		if likely!(result > 0)
		{
			new_non_zero_u32(result as u32)
		}
		else if likely!(result == -1)
		{
			unexpected_error!(fcntl, F_GETPIPE_SZ, SystemCallErrorNumber::from_errno_panic())
		}
		else
		{
			unexpected_result!(fcntl, F_GETPIPE_SZ, result)
		}
	}

	/// Will never exceed `i32::MAX as usize`.
	#[inline(always)]
	fn get_number_of_unread_bytes(&self) -> usize
	{
		let mut count: i32 = unsafe_uninitialized();
		let result = unsafe { ioctl(self.as_raw_fd(), FIONREAD, &mut count) };
		if likely!(result == 0)
		{
			count as usize
		}
		else if likely!(result == -1)
		{
			unexpected_error!(ioctl, FIONREAD, SystemCallErrorNumber::from_errno_panic())
		}
		else
		{
			unexpected_result!(ioctl, FIONREAD, result)
		}
	}
}
