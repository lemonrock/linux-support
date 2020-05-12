// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents the receiving half of a pipe.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceivePipeFileDescriptor(RawFd);

impl Drop for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0 != Self::StandardInFileDescriptor
		{
			self.0.close()
		}
	}
}

impl AsRawFd for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for ReceivePipeFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ReceivePipeFileDescriptor
{
}

impl OnDiskFileDescriptor for ReceivePipeFileDescriptor
{
}

impl PipeLikeFileDescriptor for ReceivePipeFileDescriptor
{
}

impl PipeFileDescriptor for ReceivePipeFileDescriptor
{
	#[inline(always)]
	fn clone_for_child_process(&self) -> Self
	{
		unsafe { transmute_copy(self) }
	}
}

impl SpliceSender for ReceivePipeFileDescriptor
{
}

impl VectoredRead for ReceivePipeFileDescriptor
{
	vectored_read!();
}

impl Read for ReceivePipeFileDescriptor
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

		let result = unsafe { read(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, length) };

		if likely!(result > 0)
		{
			Ok(result as usize)
		}
		else
		{
			use self::ErrorKind::*;

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

impl ReceivePipeFileDescriptor
{
	const StandardInFileDescriptor: RawFd = 0;

	/// Opens a pipe (FIFO) named in the file system suitable for receiving data from.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	#[inline(always)]
	pub fn open_fifo_for_receive(fifo_file_path: impl AsRef<Path>) -> Result<Self, SpecialFileOpenError>
	{
		SendPipeFileDescriptor::open_fifo(fifo_file_path, O_RDONLY, Self).map(|optional| optional.expect("ENXIO should not occur when open() is used with the flag O_RDONLY"))
	}

	/// Creates a new pipe.
	///
	/// Identical functionality is provided by `SendPipeFileDescriptor::new_anonymous_pipe()`.
	#[inline(always)]
	pub fn new_anonymous_pipe() -> Result<(SendPipeFileDescriptor, Self), CreationError>
	{
		SendPipeFileDescriptor::new_anonymous_pipe()
	}

	/// Wraps the standard in pipe.
	#[inline(always)]
	pub fn standard_in() -> Self
	{
		Self(Self::StandardInFileDescriptor)
	}

	/// Uses Linux's `splice()` functionality to move data.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Non-blocking.
	///
	/// `more_is_coming_hint` is used to hint that more data may be sent to `splice_to` soon.
	#[inline(always)]
	pub fn splice_to(&self, splice_from: &impl SpliceSender, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<usize, StructReadError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}

		let fd_in = splice_from.as_raw_fd();
		debug_assert_ne!(fd_in, self.0, "Can not splice to self");

		let fd_out = self.0;
		
		const CommonFlags: c_uint = SPLICE_F_MOVE | SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { splice(fd_in, null_mut(), fd_out, null_mut(), maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructReadError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,

					EINTR => Interrupted,

					EBADF => panic!("One or both file descriptors are not valid, or do not have proper read-write mode"),
					EINVAL => panic!("The target filesystem doesn't support splicing; or the target file is opened in append mode; or neither of the file descriptors refers to a pipe; or an offset was given for nonseekable device (eg, a pipe); or `fd_in` and `fd_out` refer to the same pipe"),
					ESPIPE => panic!("Either `off_in` or `off_out` was not `NULL`, but the corresponding file descriptor refers to a pipe"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Uses Linux's `splice()` functionality to move data to an offset within a File.
	///
	/// (To move using the File's current offset, use `ReceivePipeFileDescriptor::splice_from()` whose `splice_to` can be a File).
	///
	/// Returns the number of bytes transferred and the updated offset.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Non-blocking.
	///
	/// `more_is_coming_hint` is used to hint that more data may be sent to `splice_to` soon.
	#[inline(always)]
	pub fn splice_from_file_offset(&self, splice_from: &File, mut splice_from_offset: i64, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<(usize, i64), StructReadError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok((0, splice_from_offset))
		}
		
		let fd_in = splice_from.as_raw_fd();
		debug_assert_ne!(fd_in, self.0, "Can not splice to self");
		
		let fd_out = self.0;

		const CommonFlags: c_uint = SPLICE_F_MOVE | SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { splice(fd_in, &mut splice_from_offset, fd_out, null_mut(), maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok((result as usize, splice_from_offset))
		}
		else if likely!(result == -1)
		{
			use self::StructReadError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,

					EINTR => Interrupted,

					EBADF => panic!("One or both file descriptors are not valid, or do not have proper read-write mode"),
					EINVAL => panic!("The target filesystem doesn't support splicing; or the target file is opened in append mode; or neither of the file descriptors refers to a pipe; or an offset was given for nonseekable device (eg, a pipe); or `fd_in` and `fd_out` refer to the same pipe"),
					ESPIPE => panic!("Either `off_in` or `off_out` was not `NULL`, but the corresponding file descriptor refers to a pipe"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Uses Linux's `tee()` functionality to zero copy data.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Non-blocking.
	///
	/// `more_is_coming_hint` is used to hint that more data may be sent to `tee_to` soon.
	#[inline(always)]
	pub fn tee_from(&self, tee_from: &impl SpliceSender, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<usize, StructReadError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}
		
		let fd_in = tee_from.as_raw_fd();
		debug_assert_ne!(fd_in, self.0, "Can not splice to self");
		
		let fd_out = self.0;

		const CommonFlags: c_uint = SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { tee(fd_in, fd_out, maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructReadError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,

					EINTR => Interrupted,

					EINVAL => panic!("`fd_in` and `fd_out` does not refer to a pipe; or `fd_in` and `fd_out` refer to the same pipe"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}
}
