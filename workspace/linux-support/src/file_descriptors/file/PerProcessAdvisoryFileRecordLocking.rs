// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Advisory per-process file record locks.
///
/// These are POSIX file record locks.
///
/// Locks are associated with a process.
/// Locks are automatically released when a process terminates.
/// Locks are not inherited by a child created via `fork()`, but are preserved across an `execve()`.
///
/// This uses a lower-level wrapper than `lockf()` but uses the same underlying functionality.
///
/// Locks obtained this way are compatible with locks obtained by other processes using `lockf()`.
///
/// A record lock is also known as a:-
///
/// * byte-range lock
/// * file-segment lock
/// * file-region lock
///
/// There are some important caveats, which are avoid if using `OpenFileDescriptionAdvisoryFileRecordLocking` instead:-
///
/// * If a process closes *any* file descriptor referring to a file, then all of the process's locks on that file are released, regardless  of the file descriptor(s) on which the locks were obtained. This is *bad*: it means that a process can lose its locks on a file such as `/etc/passwd` or `/etc/mtab` when for some reason a library function decides to open, read, and close the same file.
/// *  The threads in a process share locks. In other words, a multithreaded program can't use record locking to ensure that threads don't simultaneously access the same region of a file.
#[deprecated(since="0.0.0", note="Prefer the use of OpenFileDescriptionAdvisoryFileRecordLocking")]
pub trait PerProcessAdvisoryFileRecordLocking: AsRawFd + Seek
{
	/// Acquire a per-process record lock (non-blocking).
	///
	/// Bytes past the end of the file may be locked, but not bytes before the start of the file.
	/// `length` can be negative, zero or positive.
	///
	/// Returns `Ok(true)` if the lock was acquired or `Ok(false)` if a conflicting lock is held elsewhere.
	/// Returns `Err(())` if the Linux kernel would run out of memory or lock resources.
	#[deprecated(since="0.0.0", note="Prefer the use of acquire_open_file_description_record_lock_on_portion_of_a_file_non_blocking")]
	#[inline(always)]
	fn acquire_advisory_per_process_record_lock_on_portion_of_a_file_non_blocking(&self, advisory_file_record_lock: AdvisoryFileRecordLock, start: ExtendedSeekFrom, length: i64) -> Result<bool, ()>
	{
		let l = flock::new_for_per_process(advisory_file_record_lock, start, length);

		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLK, &l) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			use self::AdvisoryFileRecordLock::*;
			match errno().0
			{
				EACCES | EAGAIN => Ok(false),
				ENOLCK => Err(()),
				EBADF => match advisory_file_record_lock
				{
					Read => panic!("File descriptor was not open for reading"),
					Write => panic!("File descriptor was not open for writing"),
				},
				EOVERFLOW => panic!("The cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the smallest or, if l_len is non-zero, the largest offset of any byte in the requested segment cannot be represented correctly in an object of type off_t"),
				EINVAL => panic!("The cmd argument is invalid, or the cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the data pointed to by arg is not valid, or `fildes` refers to a file that does not support locking"),
				unexpected @ _ => panic!("Unexpected error `{}` from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from fcntl()", result)
		}
	}

	/// Acquire a per-process record lock (blocking).
	///
	/// Bytes past the end of the file may be locked, but not bytes before the start of the file.
	/// `length` can be negative, zero or positive.
	///
	/// Returns `Ok(true)` if the lock was acquired or `Ok(false)` if a conflicting lock is held elsewhere and the wait was interrupted by a signal.
	/// Returns `Err(true)` if the Linux kernel detected a deadlock would occur.
	/// Returns `Err(false)` if the Linux kernel would run out of memory or lock resources.
	#[deprecated(since="0.0.0", note="Prefer the use of acquire_open_file_description_record_lock_on_portion_of_a_file_blocking")]
	#[inline(always)]
	fn acquire_advisory_per_process_record_lock_on_portion_of_a_file_blocking(&self, advisory_file_record_lock: AdvisoryFileRecordLock, start: ExtendedSeekFrom, length: i64) -> Result<bool, bool>
	{
		let l = flock::new_for_per_process(advisory_file_record_lock, start, length);

		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLKW, &l) };

		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			use self::AdvisoryFileRecordLock::*;
			match errno().0
			{
				EDEADLK => Err(true),
				ENOLCK => Err(false),
				EINTR => Ok(false),
				EBADF => match advisory_file_record_lock
				{
					Read => panic!("File descriptor was not open for reading"),
					Write => panic!("File descriptor was not open for writing"),
				},
				EOVERFLOW => panic!("The cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the smallest or, if l_len is non-zero, the largest offset of any byte in the requested segment cannot be represented correctly in an object of type off_t"),
				EINVAL => panic!("The cmd argument is invalid, or the cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the data pointed to by arg is not valid, or `fildes` refers to a file that does not support locking"),
				unexpected @ _ => panic!("Unexpected error `{}` from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from fcntl()", result)
		}
	}

	/// Release a per-process record lock (non-blocking).
	///
	/// Bytes past the end of the file may be unlocked, but not bytes before the start of the file.
	/// `length` can be negative, zero or positive.
	///
	/// Returns `true` if the lock was released and `false` if it wasn't.
	#[deprecated(since="0.0.0", note="Prefer the use of release_advisory_open_file_description_record_lock_on_portion_of_a_file_non_blocking")]
	#[inline(always)]
	fn release_advisory_per_process_record_lock_on_portion_of_a_file_non_blocking(&self, start: ExtendedSeekFrom, length: i64) -> bool
	{
		let l = flock::new_for_per_process_lock_type(F_UNLCK, start, length);

		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLK, &l) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EACCES | EAGAIN => false,
				EBADF => panic!("File descriptor was not valid"),
				EOVERFLOW => panic!("The cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the smallest or, if l_len is non-zero, the largest offset of any byte in the requested segment cannot be represented correctly in an object of type off_t"),
				EINVAL => panic!("The cmd argument is invalid, or the cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the data pointed to by arg is not valid, or `fildes` refers to a file that does not support locking"),
				unexpected @ _ => panic!("Unexpected error `{}` from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from fcntl()", result)
		}
	}

	/// Release a per-process record lock (blocking).
	///
	/// Bytes past the end of the file may be unlocked, but not bytes before the start of the file.
	/// `length` can be negative, zero or positive.
	///
	/// Returns `true` if the lock was released and `false` if it wasn't (because the wait was interrupted by a signal).
	#[deprecated(since="0.0.0", note="Prefer the use of release_advisory_open_file_description_record_lock_on_portion_of_a_file_blocking")]
	#[inline(always)]
	fn release_advisory_per_process_record_lock_on_portion_of_a_file_blocking(&self, start: ExtendedSeekFrom, length: i64) -> bool
	{
		let l = flock::new_for_per_process_lock_type(F_UNLCK, start, length);

		let result = unsafe { fcntl(self.as_raw_fd(), F_SETLKW, &l) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINTR => false,
				EBADF => panic!("File descriptor was not open or appropriate"),
				EOVERFLOW => panic!("The cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the smallest or, if l_len is non-zero, the largest offset of any byte in the requested segment cannot be represented correctly in an object of type off_t"),
				EINVAL => panic!("The cmd argument is invalid, or the cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the data pointed to by arg is not valid, or `fildes` refers to a file that does not support locking"),
				unexpected @ _ => panic!("Unexpected error `{}` from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from fcntl()", result)
		}
	}

	/// Tests if a per-process record lock can be acquired.
	///
	/// If it can not, returns information about why.
	/// This may be because a per-process or open file descriptor lock is held.
	///
	/// Bytes past the end of the file may be locked, but not bytes before the start of the file.
	/// `length` can be negative, zero or positive.
	///
	/// Returns `Ok(())` if the per-process record lock can be acquired.
	/// Returns `Err((advisory_record_lock, start, length, process))` if it can not, where the details are of another lock in place.
	/// `process` is `Some()` if the other lock is a per-process advisory file record lock (`PerProcessAdvisoryFileRecordLocking`), and `None` if it is an open file description advisory file record lock (`OpenFileDescriptionAdvisoryFileRecordLocking`).
	#[deprecated(since="0.0.0", note="Prefer the use of test_if_an_advisory_open_file_description_record_lock_on_portion_of_a_file_can_be_acquired")]
	#[inline(always)]
	fn test_if_an_advisory_per_process_record_lock_on_portion_of_a_file_can_be_acquired(&self, advisory_file_record_lock: AdvisoryFileRecordLock, start: ExtendedSeekFrom, length: i64) -> Result<(), (AdvisoryFileRecordLock, ExtendedSeekFrom, i64, Option<ProcessIdentifierChoice>)>
	{
		let l = flock::new_for_per_process(advisory_file_record_lock, start, length);

		let result = unsafe { fcntl(self.as_raw_fd(), F_GETLK, &l) };

		if likely!(result == 0)
		{
			l.process_test()
		}
		else if likely!(result == -1)
		{
			use self::AdvisoryFileRecordLock::*;
			match errno().0
			{
				EBADF => match advisory_file_record_lock
				{
					Read => panic!("File descriptor was not open for reading"),
					Write => panic!("File descriptor was not open for writing"),
				},
				EOVERFLOW => panic!("The cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the smallest or, if l_len is non-zero, the largest offset of any byte in the requested segment cannot be represented correctly in an object of type off_t"),
				EINVAL => panic!("The cmd argument is invalid, or the cmd argument is F_GETLK, F_SETLK, or F_SETLKW and the data pointed to by arg is not valid, or `fildes` refers to a file that does not support locking"),
				unexpected @ _ => panic!("Unexpected error `{}` from fcntl()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result `{}` from fcntl()", result)
		}
	}
}
