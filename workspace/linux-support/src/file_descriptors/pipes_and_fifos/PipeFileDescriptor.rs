// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Extensions for a pipe file descriptor to make it useful for clone'd processes.
pub trait PipeFileDescriptor: FileDescriptor
{
	/// Clones a pipe file descriptor so the pipe is accessible in a child process.
	fn clone_for_child_process(&self) -> Self;

	/// Changes the pipe capacity.
	///
	/// Proceses with the capability `CAP_SYS_RESOURCE` may not set the capacity to greater than `Self::maximum_capacity()`.
	///
	/// The value used may be rounded up by the Linux kernel (currently, the allocation is the next higher power-of-two page-size multiple of the requested size).
	///
	/// Returns an error if:-
	///
	/// * the caller tries to use a value greater than `Self::maximum_capacity()` without having the capability `CAP_SYS_RESOURCE`;
	/// * the caller tries to set a capacity lower than the number of bytes currently in the pipe.
	fn change_capacity(&self, new_capacity: NonZeroU32) -> Result<NonZeroU32, ChangeCapacityError>
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_SETPIPE_SZ, new_capacity.get() as i32) };
		if likely!(result > 0)
		{
			Ok(unsafe { NonZeroU32::new_unchecked(result as u32) })
		}
		else if likely!(result == 1)
		{
			use self::ChangeCapacityError::*;
			match errno().0
			{
				EPERM => Err(PermissionDenied),
				EBUSY => Err(WouldReduceCapacityBelowThatInUse),

				unexpected @ _ => panic!("Unexpected error {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from fcntl(_, F_GETPIPE_SZ)", result)
		}
	}

	/// Get current capacity.
	fn get_capacity(&self) -> NonZeroU32
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_GETPIPE_SZ) };
		if likely!(result > 0)
		{
			unsafe { NonZeroU32::new_unchecked(result as u32) }
		}
		else if likely!(result == 1)
		{
			panic!("Unexpected error {}", errno());
		}
		else
		{
			unreachable!("Unexpected result {} from fcntl(_, F_GETPIPE_SZ)", result)
		}
	}

	/// Value of `/proc/sys/fs/pipe-max-size`.
	///
	/// The default is 1Mb.
	#[inline(always)]
	fn maximum_capacity(proc_path: &ProcPath) -> usize
	{
		proc_path.sys_fs_file_path("pipe-max-size").read_value().unwrap()
	}

	/// Value of `/proc/sys/fs/pipe-max-size`.
	///
	/// The default is 1Mb.
	#[inline(always)]
	fn set_maximum_capacity(proc_path: &ProcPath, maximum_capacity: u32)
	{
		assert_effective_user_id_is_root("Change /proc/sys/fs/pipe-max-size");

		proc_path.sys_fs_file_path("pipe-max-size").write_value(maximum_capacity).unwrap()
	}
}
