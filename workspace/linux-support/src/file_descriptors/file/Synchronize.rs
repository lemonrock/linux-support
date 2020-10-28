// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Synchronize:-
///
/// * file range;
/// * just data;
/// * data and metadata;
/// * entire file system hosting file;
/// * everything in Linux.
pub trait Synchronize: AsRawFd + Seek + FileExt
{
	/// This function permits fine control when synchronizing the open file referred to by the file with disk.
	///
	/// `offset` is the starting byte of the file range to be synchronized.
	///
	/// `number_of_bytes` specifies the length of the range to be synchronized, in bytes; if `number_of_bytes` is zero, then all bytes from offset through to the end of file are synchronized.
	/// Synchronization is in units of the system page size: offset is rounded down to a page boundary; `offset + number_of_bytes - 1` is rounded up to a page boundary.
	///
	/// If `synchronize_file_range_flags` is empty, then nothing happens but a system call is wasted.
	///
	///
	/// # Warning
	///
	/// This system call is extremely dangerous.
	/// None of these operations writes out the file's metadata.
	/// Therefore, unless the application is strictly performing overwrites of already-instantiated disk blocks, there are no guarantees that the data will be available after a crash.
	/// There is no user interface to know if a write is purely an overwrite.
	/// On file systems using copy-on-write semantics (eg, btrfs) an overwrite of existing allocated blocks is impossible.
	/// When writing into preallocated space, many file systems also require calls into the block allocator, which this system call does not sync out to disk.
	/// This system call does not flush disk write caches and thus does not provide any data integrity on systems with volatile disk write caches.
	///
	/// Returns `Ok(())` on success.
	/// Returns `Err(true)` if a recoverable error was present (I/O failed, out of disk space).
	/// Returns `Err(false)` if the kernel was out of memory
	#[inline(always)]
	fn synchronize_file_range(&self, offset: u64, number_of_bytes: u64, synchronize_file_range_flags: SynchronizeFileRangeFlags) -> Result<(), bool>
	{
		let result = unsafe { sync_file_range(self.as_raw_fd(), offset as i64, number_of_bytes as i64, synchronize_file_range_flags.bits) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EIO => Err(true),
				ENOMEM => Err(false),
				ENOSPC => Err(true),
				
				EBADF => panic!("fd is not a valid file descriptor"),
				EINVAL => panic!("flags specifies an invalid bit; or offset or nbytes is invalid"),
				ESPIPE => panic!("fd refers to something other than a regular file, a block device, a directory, or a symbolic link"),
				
				unexpected @ _ => panic!("Unexpected error {} for sync_file_range()", unexpected)
			}

		}
		else
		{
			unreachable_code(format_args!("sync_file_range() returned a result of {}", result))
		}
	}

	/// Similar to `Write.flush()` but with more granularity.
	///
	/// `Err(true)` - some sort of I/O error occurred.
	/// `Err(false)` - out of disk space.
	#[inline(always)]
	fn synchronize_data_only(&self) -> Result<(), bool>
	{
		let result = unsafe { fdatasync(self.as_raw_fd()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("fd is not a valid file descriptor"),
				EROFS | EINVAL => panic!("fd is bound to a special file (e.g., a pipe, FIFO, or socket) which does not support synchronization"),
				EIO => Err(true),
				ENOSPC | EDQUOT => Err(false),
				unexpected @ _ => panic!("Unexpected error {} for fdatasync()", unexpected)
			}

		}
		else
		{
			unreachable_code(format_args!("fdatasync() returned a result of {}", result))
		}
	}

	/// Similar to `Write.flush()` but with more granularity.
	///
	/// `Err(true)` - some sort of I/O error occurred.
	/// `Err(false)` - out of disk space.
	#[inline(always)]
	fn synchronize_data_and_metadata(&self) -> Result<(), bool>
	{
		let result = unsafe { fsync(self.as_raw_fd()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("fd is not a valid file descriptor"),
				EROFS | EINVAL => panic!("fd is bound to a special file (e.g., a pipe, FIFO, or socket) which does not support synchronization"),
				EIO => Err(true),
				ENOSPC | EDQUOT => Err(false),
				unexpected @ _ => panic!("Unexpected error {} for fsync()", unexpected)
			}

		}
		else
		{
			unreachable_code(format_args!("fsync() returned a result of {}", result))
		}
	}

	/// Synchronizes all data for the filesystem containing the file referred to by `self`.
	///
	/// Available since Linux 2.6.39.
	#[inline(always)]
	fn synchronize_filesystem(&self)
	{
		let result = unsafe { syncfs(self.as_raw_fd()) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("fd is not a valid file descriptor"),
				unexpected @ _ => panic!("Unexpected error {} for syncfs()", unexpected)
			}

		}
		else
		{
			unreachable_code(format_args!("syncfs() returned a result of {}", result))
		}
	}

	/// Synchronizes all data for the filesystem containing the file referred to by `self`.
	#[inline(always)]
	fn synchronize_everything()
	{
		unsafe { sync() }
	}
}
