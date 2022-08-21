// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Fast logic that avoids memory copies inside the Linux kernel to copy from one file to another.
///
/// Use this trait in conjunction with `Sparseness` to efficiently copy files from one location to another (by finding out where there are zero-byte holes, and so not copying them).
///
/// If the underlying file system supports Copy-on-Write, see the `CopyOnWrite` trait for more efficient copying still.
pub trait CopyFileRange: AsRawFd + Seek + FileExt
{
	/// Panics if `from_offset + length` exceeds `usize::MAX`.
	/// Panics if `to_offset + length` exceeds `usize::MAX`.
	/// Panics if `from` and `to` refer to the same file and `from_offset + length` overlaps with `to_offset + length`.
	///
	/// `from_offset`, if `None`, causes reads to start from the current (seek) position in the file read from (`self`).
	/// After a copy, the seek position of `self` will have changed by the numbers of bytes `Ok(number of bytes copied)`.
	///
	/// `from_offset`, if `Some(offset)`, causes reads to start from the position in the file read from `offset`.
	/// It will be updated by `Ok(number of bytes copied)` after a copy, but the file read from 's current (seek) position will be unchanged.
	///
	/// `to_offset`, if `None`, causes writes to start from the current (seek) position in the file written `to`.
	/// After a copy, the seek position of `self` will have changed by the numbers of bytes `Ok(number of bytes copied)`.
	///
	/// `to_offset`, if `Some(offset)`, causes writes to start from the position in the file written to `offset`.
	/// It will be updated by `Ok(number of bytes copied)` after a copy, but the file written to 's current (seek) position will be unchanged.
	///
	/// Returns `Ok(number of bytes copied)` on success.
	/// `Err(true)` if a non-permanent failure (out of kernel memory, low-level IO failure) occurred.
	/// `Err(false)` if a permanent failure (out of disk space, and, before Linux 5.3, files are on different filesystems) occurred.
	#[inline(always)]
	fn copy_file_range_to<CFR: CopyFileRange>(&self, from_offset: Option<&mut i64>, to: &CFR, to_offset: Option<&mut i64>, length: usize) -> Result<usize, bool>
	{
		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		const FlagsAreUnused: u32 = 0;
		let off_in = from_offset.map(|offset| offset as *mut i64).unwrap_or(null_mut());
		let off_out = to_offset.map(|offset| offset as *mut i64).unwrap_or(null_mut());
		let result = unsafe { copy_file_range(self.as_raw_fd(), off_in, to.as_raw_fd(), off_out, length, FlagsAreUnused) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EIO | ENOMEM => Err(true),
				ENOSPC | EXDEV => Err(false),

				EBADF => panic!("Either fd_in, fd_out or both file descriptors are not valid. Or fd_in is not open for reading. Or fd_out is not open for writing. Or the O_APPEND flag is set for the open file description referred to by the file descriptor fd_out."),
				EFBIG => panic!("An attempt was made to write at a position past the maximum file offset the kernel supports. Or an attempt was made to write a range that exceeds the allowed maximum file size (The maximum file size differs between filesystem domain and can be different from the maximum allowed file offset). Or an attempt was made to write beyond the process's file size resource limit.  This may also result in the process receiving a SIGXFSZ signal"),
				EINVAL => panic!("The flags argument is not 0. Or fd_in and fd_out refer to the same file and the source and target ranges overlap. Or either fd_in or fd_out (or both) is not a regular file."),
				EPERM => panic!("fd_out refers to an immutable file"),
				EOVERFLOW => panic!("The requested source or destination range (offset + length) is too large to represent in the specified data types (ie exceeds usize::MAX)"),
				EISDIR => panic!("Either fd_in or fd_out refers to a directory"),
				ETXTBSY => panic!("Either fd_in or fd_out refers to an active swap file"),

				unexpected_error @ _ => unexpected_error!(fcntl, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(copy_file_range, result)
		}
	}
}
