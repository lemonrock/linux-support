// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File locks.
pub trait FileLock: AsRawFd + Seek
{
	/// The value of `portion_offset` is influenced by the current file seek position (called `seek_position` here) of the file:-
	///
	/// * if `portion_offset` is positive, then this exclusive locks the portion (range) of the file from `seek_position` (inclusive) to the `seek_position + portion_offset - 1` (inclusive).
	/// * if `portion_offset` is negative, then this exclusive locks the portion (range) of the file from `seek_position - abs(portion_offset)` (inclusive) to `seek_position - 1` (inclusive).
	/// * if `portion_offset` is zero, then this exclusive locks the portion (range) of the file from `seek_position` to end-of-file (EOF), for the current and any future EOF value.
	///
	/// Thus, to take a whole-of-file lock, call `self.seek(SeekFrom::Start(0))` then `self.take_advisory_exclusive_posix_lock_on_portion_of_a_file(0)`.
	///
	/// Returns `Ok(true)` if the lock was taken and `Ok(false)` if it wasn't.
	/// Returns `Err(true)` if a potential deadlock was detected.
	/// Returns `Err(false)` if the kernel ran out of memory for allocating lock records.
	#[inline(always)]
	fn take_advisory_exclusive_posix_lock_on_portion_of_a_file_blocking(&self, portion_offset: i64) -> Result<bool, bool>
	{
		let result = unsafe { lockf(self.as_raw_fd(), F_LOCK, portion_offset) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => Ok(false),
				ENOLCK => Err(false),
				EDEADLK => Err(true),
				EACCES | EAGAIN => panic!("Try again but call made with blocking variant"),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("An invalid operation was specified in cmd"),
				unexpected @ _ => panic!("Unexpected {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from lockf()", result)
		}
	}

	/// The value of `portion_offset` is influenced by the current file seek position (called `seek_position` here) of the file:-
	///
	/// * if `portion_offset` is positive, then this exclusive locks the portion (range) of the file from `seek_position` (inclusive) to the `seek_position + portion_offset - 1` (inclusive).
	/// * if `portion_offset` is negative, then this exclusive locks the portion (range) of the file from `seek_position - abs(portion_offset)` (inclusive) to `seek_position - 1` (inclusive).
	/// * if `portion_offset` is zero, then this exclusive locks the portion (range) of the file from `seek_position` to end-of-file (EOF), for the current and any future EOF value.
	///
	/// Thus, to take a whole-of-file lock, call `self.seek(SeekFrom::Start(0))` then `self.take_advisory_exclusive_posix_lock_on_portion_of_a_file(0)`.
	///
	/// Returns `Ok(true)` if the lock was taken and `Ok(false)` if it wasn't.
	/// Returns `Err(true)` if a potential deadlock was detected.
	/// Returns `Err(false)` if the kernel ran out of memory for allocating lock records.
	#[inline(always)]
	fn take_advisory_exclusive_posix_lock_on_portion_of_a_file_non_blocking(&self, portion_offset: i64) -> Result<bool, bool>
	{
		let result = unsafe { lockf(self.as_raw_fd(), F_TLOCK, portion_offset) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => panic!("Should not be possible to interrupt with a signal as non-blocking"),
				ENOLCK => Err(false),
				EDEADLK => Err(true),
				EACCES | EAGAIN => Ok(false),
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("An invalid operation was specified in cmd"),
				unexpected @ _ => panic!("Unexpected {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from lockf()", result)
		}
	}

	/// The value of `portion_offset` is influenced by the current file seek position (called `seek_position` here) of the file:-
	///
	/// * if `portion_offset` is positive, then this exclusive locks the portion (range) of the file from `seek_position` (inclusive) to the `seek_position + portion_offset - 1` (inclusive).
	/// * if `portion_offset` is negative, then this exclusive locks the portion (range) of the file from `seek_position - abs(portion_offset)` (inclusive) to `seek_position - 1` (inclusive).
	/// * if `portion_offset` is zero, then this exclusive locks the portion (range) of the file from `seek_position` to end-of-file (EOF), for the current and any future EOF value.
	///
	/// Thus, to release a whole-of-file lock, call `self.seek(SeekFrom::Start(0))` then `self.take_advisory_exclusive_posix_lock_on_portion_of_a_file(0)`.
	///
	/// Returns `true` if the lock was released and `false` if it wasn't (eg because the file is memory-mapped).
	#[inline(always)]
	fn release_advisory_exclusive_posix_lock_on_portion_of_a_file(&self, portion_offset: i64) -> bool
	{
		let result = unsafe { lockf(self.as_raw_fd(), F_ULOCK, portion_offset) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => panic!("Should not be possible to be interrupted"),
				ENOLCK => panic!("Should not be possible to run out of memory"),
				EDEADLK => panic!("Deadlock should not occur"),
				EACCES | EAGAIN => false,
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("An invalid operation was specified in cmd"),
				unexpected @ _ => panic!("Unexpected {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from lockf()", result)
		}
	}

	/// The value of `portion_offset` is influenced by the current file seek position (called `seek_position` here) of the file:-
	///
	/// * if `portion_offset` is positive, then this exclusive locks the portion (range) of the file from `seek_position` (inclusive) to the `seek_position + portion_offset - 1` (inclusive).
	/// * if `portion_offset` is negative, then this exclusive locks the portion (range) of the file from `seek_position - abs(portion_offset)` (inclusive) to `seek_position - 1` (inclusive).
	/// * if `portion_offset` is zero, then this exclusive locks the portion (range) of the file from `seek_position` to end-of-file (EOF), for the current and any future EOF value.
	///
	/// Thus, to test a whole-of-file lock, call `self.seek(SeekFrom::Start(0))` then `self.take_advisory_exclusive_posix_lock_on_portion_of_a_file(0)`.
	///
	/// Returns `true` if the file has the lock.
	#[inline(always)]
	fn has_advisory_exclusive_posix_lock_on_portion_of_a_file(&self, portion_offset: i64) -> bool
	{
		let result = unsafe { lockf(self.as_raw_fd(), F_TEST, portion_offset) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => panic!("Should not be possible to be interrupted"),
				ENOLCK => panic!("Should not be possible to run out of memory"),
				EDEADLK => panic!("Deadlock should not occur"),
				EACCES | EAGAIN => false,
				EBADF => panic!("fd is not an open file descriptor"),
				EINVAL => panic!("An invalid operation was specified in cmd"),
				unexpected @ _ => panic!("Unexpected {}", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from lockf()", result)
		}
	}

	/// Uses `flock()` system call.
	///
	/// This *DOES NOT WORK* on files on NFS shares.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be applied, although, however, it can return without taking the lock because of interruption by a signal.
	///
	/// Returns `Ok(true)` if the lock was applied and `Ok(false)` if it wasn't.
	/// Returns `Err(())` if the kernel ran out of memory for allocating lock records.
	#[inline(always)]
	fn take_advisory_lock_blocking(&self, advisory_lock: AdvisoryLock) -> Result<bool, ()>
	{
		let result = unsafe { flock(self.as_raw_fd(), advisory_lock as i32) };

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
	/// This *DOES NOT WORK* on files on NFS shares.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Does not block until the lock can be applied.
	/// Returns `Ok(true)` if the lock was applied and `Ok(false)` if it wasn't.
	/// Returns `Err(())` if the kernel ran out of memory for allocating lock records.
	#[inline(always)]
	fn take_advisory_lock_non_blocking(&self, advisory_lock: AdvisoryLock) -> Result<bool, ()>
	{
		let result = unsafe { flock(self.as_raw_fd(), (advisory_lock as i32) | LOCK_NB) };

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
	/// This *DOES NOT WORK* on files on NFS shares.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be released.
	/// Returns `true` if the lock was applied and `false` if it wasn't because a signal interrupted the lock's release.
	#[inline(always)]
	fn release_advisory_lock_blocking(&self) -> bool
	{
		let result = unsafe { flock(self.as_raw_fd(), LOCK_UN) };

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

	/// Uses `flock()` system call.
	///
	/// This *DOES NOT WORK* on files on NFS shares.
	///
	/// If the underlying file descriptor is duplicated (including inheritated unclosed) by a child process, the lock is also inheritated.
	/// Likewise, if a lock is released on a file descriptor, it is released on all duplicates.
	/// This is *NOT* true if two separate file descriptors are created using `open()`.
	///
	/// Blocks until the lock can be released.
	/// Returns `true` if the lock was applied and `false` if it wasn't because a signal interrupted the lock's release.
	#[inline(always)]
	fn release_advisory_lock_non_blocking(&self) -> bool
	{
		let result = unsafe { flock(self.as_raw_fd(), LOCK_UN | LOCK_NB) };

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
}
