// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a block device for reading and writing to.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockDeviceFileDescriptor(RawFd);

impl Drop for BlockDeviceFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for BlockDeviceFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for BlockDeviceFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for BlockDeviceFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for BlockDeviceFileDescriptor
{
}

impl OnDiskFileDescriptor for BlockDeviceFileDescriptor
{
}

impl SeekableFileDescriptor for BlockDeviceFileDescriptor
{
}

impl SpliceRecipient for BlockDeviceFileDescriptor
{
}

impl SpliceSender for BlockDeviceFileDescriptor
{
}

impl VectoredRead for BlockDeviceFileDescriptor
{
	vectored_read!();
}

impl VectoredWrite for BlockDeviceFileDescriptor
{
	vectored_write!();
}

impl Read for BlockDeviceFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `UnexpectedEof`
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `Other` (which is for when the kernel reports `ENOMEM`, ie it is out of memory).
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		let length = buf.len();

		debug_assert!(length < std::isize::MAX as usize, "length can not exceed SSIZE_MAX for read()");

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { libc::read(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use crate::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						UnexpectedEof
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							EIO => Other,
							EBADF => panic!("The argument `sockfd` is an invalid descriptor"),
							EFAULT => panic!("The receive buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							EISDIR => panic!("`fd` refers to a directory"),
							_ => unreachable!(),
						}
					}
					else
					{
						unreachable!()
					}
				)
			)
		}
	}

	#[inline(always)]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
	{
		VectoredRead::read_vectored(self, unsafe { transmute(bufs) })
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		Initializer::nop()
	}
}

impl Write for BlockDeviceFileDescriptor
{
	/// This particular implementation can only return an `io::ErrorKind` of:-
	///
	/// * `WriteZero` (implying end-of-file).
	/// * `WouldBlock`
	/// * `Interrupted`
	/// * `BrokenPipe`
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		let length = buf.len();

		if unlikely!(length == 0)
		{
			return Ok(0)
		}

		let result = unsafe { libc::write(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len()) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use crate::ErrorKind::*;

			Err
			(
				io::Error::from
				(
					if likely!(result == 0)
					{
						WriteZero
					}
					else if likely!(result == -1)
					{
						match errno().0
						{
							EAGAIN => WouldBlock,
							EINTR => Interrupted,
							EPIPE => BrokenPipe,
							ENOSPC => panic!("The device containing the file referred to by `fd` has no room for the data"),
							EBADF => panic!("The argument `fd` is an invalid descriptor"),
							EFAULT => panic!("The write buffer pointer(s) point outside the process's address space"),
							EINVAL => panic!("Invalid argument passed"),
							EDESTADDRREQ => panic!("`fd` refers to a datagram socket for which a peer address has not been set using `connect()`"),
							_ => unreachable!(),
						}
					}
					else
					{
						unreachable!()
					}
				)
			)
		}
	}

	#[inline(always)]
	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize>
	{
		VectoredWrite::write_vectored(self, unsafe { transmute(bufs) })
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}
}

impl Synchronize for BlockDeviceFileDescriptor
{
}

impl ReadAhead for BlockDeviceFileDescriptor
{
}

impl CopyFileRange for BlockDeviceFileDescriptor
{
}

impl Seek for BlockDeviceFileDescriptor
{
	#[inline(always)]
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>
	{
		use crate::SeekFrom::*;

		let (whence, start) = match pos
		{
			Start(value) => (SEEK_SET, value as i64),
			End(value) => (SEEK_END, value),
			Current(value) => (SEEK_CUR, value),
		};

		let result = unsafe { lseek(self.as_raw_fd(), start, whence as i32) };
		if likely!(result >= 0)
		{
			Ok(result as u64)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from lseek", result)
		}
	}
}

impl ExtendedSeek for BlockDeviceFileDescriptor
{
}

impl FileExt for BlockDeviceFileDescriptor
{
	#[inline(always)]
	fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>
	{
		let result = unsafe { pread(self.as_raw_fd(), buf.as_mut_ptr() as *mut _, buf.len(), offset as i64) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from pread()", result)
		}
	}

	#[inline(always)]
	fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>
	{
		let result = unsafe { pwrite(self.as_raw_fd(), buf.as_ptr() as *const _, buf.len(), offset as i64) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from pwrite()", result)
		}
	}
}

impl BlockDeviceFileDescriptor
{
	/// Opens a block device named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a block device.
	#[inline(always)]
	pub fn open_block_device(block_device_file_path: impl AsRef<Path>) -> Result<Self, SpecialFileOpenError>
	{
		Self::open_block_device_internal(block_device_file_path, 0)
	}

	#[inline(always)]
	pub(crate) fn open_block_device_internal(block_device_file_path: impl AsRef<Path>, additional_flags: c_int) -> Result<Self, SpecialFileOpenError>
	{
		let fifo_path = CString::new(path_bytes_without_trailing_nul(&block_device_file_path)).unwrap();

		const CommonFlags: c_int = O_RDWR | O_CLOEXEC | O_NONBLOCK;

		let result = unsafe { open(fifo_path.as_ptr(), additional_flags | CommonFlags) };
		if likely!(result != -1)
		{
			Ok(Self(result))
		}
		else
		{
			use self::CreationError::*;
			use self::SpecialFileOpenError::*;
			use self::InvalidPathReason::*;

			Err
			(
				match errno().0
				{
					EACCES => Common(PermissionDenied),
					EMFILE => Common(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),
					ENFILE => Common(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),
					ENOMEM => Common(KernelWouldBeOutOfMemory),
					EAGAIN => WouldBlock,
					EINTR => Interrupted,
					ELOOP => InvalidPath(TooManySymbolicLinks),
					ENAMETOOLONG => InvalidPath(TooLong),
					EISDIR => InvalidPath(IsADirectory),
					ENOENT => InvalidPath(DoesNotExist),
					ENOTDIR => InvalidPath(ParentComponentIsNotADirectory),
					ENODEV | EROFS | ETXTBSY => InvalidPath(ExistsButCanNotBeUsed),
					ENXIO => InvalidPath(ExistsButCanNotBeUsed),

					EDQUOT => panic!("Where `O_CREAT `is specified, the file does not exist, and the user's quota of disk blocks or inodes on the file system has been exhausted"),
					EEXIST => panic!("`pathname` already exists and `O_CREAT` and `O_EXCL` were used"),
					EFAULT => panic!("`pathname` points outside your accessible address space"),
					EFBIG | EOVERFLOW => panic!("`pathname` refers to a regular file that is too large to be opened. The usual scenario here is that an application compiled on a 32-bit platform without `-D_FILE_OFFSET_BITS=64` tried to open a file whose size exceeds `(2<<31)-1` bits; see also `O_LARGEFILE` above. This is the error specified by POSIX.1-2001; in kernels before 2.6.24, Linux gave the error `EFBIG` for this case"),
					ENOSPC => panic!("`pathname` was to be created but the device containing `pathname` has no room for the new file"),
					EPERM => panic!("The `O_NOATIME` flag was specified, but the effective user ID of the caller did not match the owner of the file and the caller was not privileged (`CAP_FOWNER`)"),

					_ => unreachable!(),
				}
			)
		}
	}
}
