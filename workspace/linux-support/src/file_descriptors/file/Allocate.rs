// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Allocates space on disk for future writes to succeed.
pub trait Allocate: AsRawFd + Seek + FileExt
{
	/// Allocates space on disk for future writes to succeed.
	///
	/// Returns an `Err(true)` if interupted by a signal or file is currently being executed.
	/// Returns an `Err(false)` if out-of-disk space.
	fn allocate(&self, offset: u64, length: NonZeroU64, mode: AllocationMode) -> Result<(), bool>
	{
		let result = unsafe { fallocate(self.as_raw_fd(), mode.bits, offset as i64, length.get() as i64) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else
		{
			match errno().0
			{
				EINTR => Err(true),
				ENOSPC => Err(false),

				// mode specifies FALLOC_FL_COLLAPSE_RANGE or FALLOC_FL_INSERT_RANGE, but the file referred to by fd is currently being executed.
				ETXTBSY => Err(true),

				EBADF => panic!("fd is not a valid file descriptor, or is not opened for writing"),
				EFBIG => panic!("`offset + length` exceeds the maximum file size"),
				EINVAL => panic!("offset was less than 0, or len was less than or equal to 0, or the underlying filesystem does not support the operation"),
				ENODEV => panic!("fd does not refer to a regular file"),
				ESPIPE => panic!("fd refers to a pipe"),
				ENOSYS => panic!("This kernel does not implement fallocate()"),
				EOPNOTSUPP => panic!("The filesystem containing the file referred to by fd does not support this operation; or the mode is not supported by the filesystem containing the file referred to by fd"),
				EPERM => panic!("The file referred to by fd is marked immutable (see chattr(1)). Or mode specifies FALLOC_FL_PUNCH_HOLE or FALLOC_FL_COLLAPSE_RANGE or FALLOC_FL_INSERT_RANGE and the file referred to by fd is marked append-only (see chattr(1)). Or the operation was prevented by a file seal; see fcntl(2)."),

				unexpected @ _ => panic!("Unexpected error {} for posix_fallocate()", unexpected),
			}
		}
	}
}
