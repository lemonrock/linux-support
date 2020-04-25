// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Per-process, legacy whole file record locking.
#[deprecated(since="0.0.0", note="Prefer the use of OpenFileDescriptionAdvisoryFileRecordLocking")]
pub trait AdvisoryWholeFileLocking: AsRawFd + Seek + FileExt
{
	/// Uses `flock()` system call.
	///
	/// These *MAY NOT WORK* on files on NFS shares and non-local file systems; Linux kernel support is inconsistent and difficult to deduce.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Does not block until the lock can be applied.
	/// Returns `Ok(true)` if the lock was applied and `Ok(false)` if it wasn't.
	/// Returns `Err(())` if the kernel ran out of memory for allocating lock records.
	#[deprecated(since="0.0.0", note="Prefer the use of acquire_advisory_open_file_description_record_lock_on_portion_of_a_file_non_blocking")]
	#[inline(always)]
	#[allow(deprecated)]
	fn acquire_advisory_whole_file_lock_non_blocking(&self, advisory_whole_file_lock: AdvisoryWholeFileLock) -> Result<bool, ()>
	{
		let result = unsafe { libc::flock(self.as_raw_fd(), (advisory_whole_file_lock as i32) | LOCK_NB) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENOLCK => Err(()),
				EWOULDBLOCK => Ok(false),
				EINTR => panic!("Should not be possible to interrupt with a signal as non-blocking"),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected @ _ => panic!("Unexpected error {} from flock()", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result {} from flock()", result)
		}
	}

	/// Uses `flock()` system call.
	///
	/// These *MAY NOT WORK* on files on NFS shares and non-local file systems; Linux kernel support is inconsistent and difficult to deduce.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be applied, although, however, it can return without taking the lock because of interruption by a signal.
	///
	/// Returns `Ok(true)` if the lock was applied and `Ok(false)` if it wasn't.
	/// Returns `Err(())` if the kernel ran out of memory for allocating lock records.
	#[deprecated(since="0.0.0", note="Prefer the use of acquire_advisory_open_file_description_record_lock_on_portion_of_a_file_blocking")]
	#[inline(always)]
	#[allow(deprecated)]
	fn acquire_advisory_whole_file_lock_blocking(&self, advisory_whole_file_lock: AdvisoryWholeFileLock) -> Result<bool, ()>
	{
		let result = unsafe { libc::flock(self.as_raw_fd(), advisory_whole_file_lock as i32) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => Ok(false),
				ENOLCK => Err(()),
				EWOULDBLOCK => panic!("The file is locked and the LOCK_NB flag was selected"),
				EINVAL => panic!("operation is invalid"),
				EBADF => panic!("fd is not an open file descriptor"),

				unexpected @ _ => panic!("Unexpected error {} from flock()", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result {} from flock()", result)
		}
	}

	/// Uses `flock()` system call.
	///
	/// These *MAY NOT WORK* on files on NFS shares and non-local file systems; Linux kernel support is inconsistent and difficult to deduce.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be released.
	/// Returns `true` if the lock was applied and `false` if it wasn't because a signal interrupted the lock's release.
	#[deprecated(since="0.0.0", note="Prefer the use of release_advisory_open_file_description_record_lock_on_portion_of_a_file_non_blocking")]
	#[inline(always)]
	fn release_advisory_whole_file_lock_non_blocking(&self) -> bool
	{
		let result = unsafe { libc::flock(self.as_raw_fd(), LOCK_UN | LOCK_NB) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EWOULDBLOCK => false,
				EINTR => panic!("Should not be possible to interrupt with a signal as non-blocking"),
				ENOLCK => panic!("the kernel ran out of memory for allocating lock records."),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected @ _ => panic!("Unexpected error {} from flock()", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result {} from flock()", result)
		}
	}

	/// Uses `flock()` system call.
	///
	/// These *MAY NOT WORK* on files on NFS shares and non-local file systems; Linux kernel support is inconsistent and difficult to deduce.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be released.
	/// Returns `true` if the lock was applied and `false` if it wasn't because a signal interrupted the lock's release.
	#[deprecated(since="0.0.0", note="Prefer the use of release_advisory_open_file_description_record_lock_on_portion_of_a_file_blocking")]
	#[inline(always)]
	fn release_advisory_whole_file_lock_blocking(&self) -> bool
	{
		let result = unsafe { libc::flock(self.as_raw_fd(), LOCK_UN) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => false,
				EWOULDBLOCK => panic!("The file is locked and the LOCK_NB flag was selected"),
				ENOLCK => panic!("the kernel ran out of memory for allocating lock records."),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("operation is invalid"),

				unexpected @ _ => panic!("Unexpected error {} from flock()", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result {} from flock()", result)
		}
	}
}
