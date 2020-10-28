// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Clones a file or ranges (parts) of file.
///
/// Files involved must exist on the same file system.
///
/// `clone_to()` is the most efficient way of copying files possible.
pub trait CopyOnWrite: AsRawFd + Seek + FileExt
{
	/// Atomically clone one file to another.
	///
	/// Also called `reflink`-ing.
	///
	/// Returns an explanation if the operation failed because the file system or files themselves do not support reflinks.
	/// Sadly the underlying Linux API does not properly differentiate invalid file descriptors from fundamental lack of support for reflinks.
	#[inline(always)]
	fn atomic_clone_to<COW: CopyOnWrite>(&self, to: &COW) -> Result<(), &'static str>
	{
		let result = unsafe { ioctl(to.as_raw_fd(), FICLONE, self.as_raw_fd()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => Err("The filesystem which src_fd resides on does not support reflink OR src_fd is not open for reading; dest_fd is not open for writing or is open for append-only writes"),
				EOPNOTSUPP => Err("This can appear if the filesystem does not support reflinking either file descriptor, or if either file descriptor refers to special inodes"),
				EXDEV => Err("dest_fd and src_fd are not on the same mounted filesystem"),

				EPERM => panic!("dest_fd is immutable"),
				ETXTBSY => panic!("One of the files is a swap file. Swap files cannot share storage"),

				EINVAL => unreachable_code(format_args!("The filesystem does not support reflinking the ranges of the given files. This error can also appear if either file descriptor represents a device, FIFO, or socket. Disk filesystems generally require the offset and length arguments to be aligned to the fundamental block size. XFS and Btrfs do not support overlapping reflink ranges in the same file.")),
				EISDIR => unreachable_code(format_args!("One of the files is a directory and the filesystem does not support shared regions in directories.")),

				_ => Err("other error from ioctl()")
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from ioctl()", result))
		}
	}

	/// Atomically clone part of a file to another file.
	///
	/// Also called `reflink`-ing.
	///
	/// Returns an explanation if the operation failed because the file system or files themselves do not support reflinks.
	/// Sadly the underlying Linux API does not properly differentiate invalid file descriptors from fundamental lack of support for reflinks.
	#[inline(always)]
	fn atomic_clone_range_to<COW: CopyOnWrite>(&self, from_logical_range_in_bytes: RangeInclusive<u64>, to: &COW, to_logical_offset: u64) -> Result<(), &'static str>
	{
		let (start, end) = from_logical_range_in_bytes.into_inner();
		let length = end - start + 1;

		let input = file_clone_range
		{
			src_fd: self.as_raw_fd() as i64,
			src_offset: start,
			src_length: length,
			dest_offset: to_logical_offset,
		};

		let result = unsafe { ioctl(to.as_raw_fd(), FICLONERANGE, &input) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => Err("The filesystem which src_fd resides on does not support reflink OR src_fd is not open for reading; dest_fd is not open for writing or is open for append-only writes"),
				EINVAL => Err("The filesystem does not support reflinking the ranges of the given files. This error can also appear if either file descriptor represents a device, FIFO, or socket. Disk filesystems generally require the offset and length arguments to be aligned to the fundamental block size. XFS and Btrfs do not support overlapping reflink ranges in the same file."),
				EOPNOTSUPP => Err("This can appear if the filesystem does not support reflinking either file descriptor, or if either file descriptor refers to special inodes"),
				EXDEV => Err("dest_fd and src_fd are not on the same mounted filesystem"),

				EPERM => panic!("dest_fd is immutable"),
				ETXTBSY => panic!("One of the files is a swap file. Swap files cannot share storage"),

				EISDIR => unreachable_code(format_args!("One of the files is a directory and the filesystem does not support shared regions in directories.")),

				_ => Err("other error from ioctl()")
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from ioctl()", result))
		}
	}

	/// `to_files_and_their_logical_offsets` can not be empty.
	///
	/// `from_logical_range_in_bytes.end` is usually limited to the first 16Mb of any file.
	/// `to_files_and_their_logical_offsets.1 + from_logical_range_in_bytes.len` is usually limited to the first 16Mb of any file.
	///
	/// On a system with a 4Kb page size, `to_files_and_their_logical_offsets.len()` must be 127 or less.
	/// (The actual constraint is `length <= (PageSizeInBytes - 24)/32`).
	#[inline(always)]
	fn deduplicate<COW: CopyOnWrite>(&self, from_logical_range_in_bytes: RangeInclusive<u64>, to_files_and_their_logical_offsets: &[(&COW, u64)]) -> Result<Deduplicate, &'static str>
	{
		let mut deduplicate = Deduplicate::new(from_logical_range_in_bytes, to_files_and_their_logical_offsets);

		let result = unsafe { ioctl(self.as_raw_fd(), FIDEDUPERANGE, deduplicate.data_pointer()) };
		if likely!(result == 0)
		{
			Ok(deduplicate)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => Err("The filesystem which src_fd resides on does not support deduplication OR src_fd is not open for reading; dest_fd is not open for writing or is open for append-only writes."),
				EINVAL => Err("The filesystem does not support deduplicating the ranges of the given files. This error can also appear if either file descriptor represents a device, FIFO, or socket. Diskfilesystems generally require the offset and length argumentsto be aligned to the fundamental block size. Neither Btrfs nor XFS support overlapping deduplication ranges in the same file"),
				ENOMEM => Err("The kernel was unable to allocate sufficient memory to perform the operation or dest_count is so large that the input argument description spans more than a single page of memory"),
				EOPNOTSUPP => Err("TThis can appear if the filesystem does not support deduplicating either file descriptor, or if either file descriptor refers to special inodes."),
				EXDEV => Err("dest_fd and src_fd are not on the same mounted filesystem"),

				EPERM => panic!("dest_fd is immutable"),
				ETXTBSY => panic!("One of the files is a swap file. Swap files cannot share storage"),

				EISDIR => unreachable_code(format_args!("One of the files is a directory and the filesystem does not support shared regions in directories.")),

				_ => Err("other error from ioctl()")
			}

		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from ioctl()", result))
		}
	}
}
