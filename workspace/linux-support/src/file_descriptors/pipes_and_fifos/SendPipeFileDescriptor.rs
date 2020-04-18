// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents the sending half of a pipe.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPipeFileDescriptor(RawFd);

impl Drop for SendPipeFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		match self.0
		{
			Self::StandardOutFileDescriptor | Self::StandardErrorFileDescriptor => (),
			_ => self.0.close(),
		}
	}
}

impl AsRawFd for SendPipeFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for SendPipeFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for SendPipeFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for SendPipeFileDescriptor
{
}

impl OnDiskFileDescriptor for SendPipeFileDescriptor
{
}

impl VectoredWrite for SendPipeFileDescriptor
{
	vectored_write!();
}

impl SpliceRecipient for SendPipeFileDescriptor
{
}

impl PipeFileDescriptor for SendPipeFileDescriptor
{
	#[inline(always)]
	fn clone_for_child_process(&self) -> Self
	{
		unsafe { transmute_copy(self) }
	}
}

impl Write for SendPipeFileDescriptor
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

		let result = unsafe { write(self.as_raw_fd(), buf.as_ptr() as *const c_void, buf.len()) };

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

impl SendPipeFileDescriptor
{
	const StandardOutFileDescriptor: RawFd = 1;

	const StandardErrorFileDescriptor: RawFd = 2;

	/// Opens a pipe (FIFO) named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	///
	/// Returns `Ok(Some(Self))` if successful.
	/// Returns `Ok(None)` if there wasn't a process already receiving from this FIFO.
	#[inline(always)]
	pub fn open_fifo_for_send(fifo_file_path: impl AsRef<Path>) -> Result<Option<Self>, SpecialFileOpenError>
	{
		Self::open_fifo(fifo_file_path, O_WRONLY, Self)
	}

	/// Opens a pipe (FIFO) named in the file system suitable for sending data to.
	///
	/// Sadly, there is no way to atomically detect if the provided path is **not** a FIFO.
	///
	/// Opens regardless of whether another process is already receiving from this FIFO.
	#[inline(always)]
	pub fn open_fifo_for_send_irrespective_of_another_process_already_having_opened_the_fifo_for_receive(fifo_file_path: impl AsRef<Path>) -> Result<Self, SpecialFileOpenError>
	{
		Self::open_fifo(fifo_file_path, O_RDWR, Self).map(|optional| optional.expect("ENXIO should not occur with O_RDWR set in open()"))
	}

	/// Creates a new pipe.
	///
	/// Identical functionality is provided by `ReceivePipeFileDescriptor::new_anonymous_pipe()`.
	#[inline(always)]
	pub fn new_anonymous_pipe() -> Result<(Self, ReceivePipeFileDescriptor), CreationError>
	{
		#[allow(deprecated)]
		let mut pipe_file_descriptors = unsafe { uninitialized() };
		let result = unsafe { pipe2(&mut pipe_file_descriptors, O_NONBLOCK | O_CLOEXEC) };
		if likely!(result == 0)
		{
			Ok((SendPipeFileDescriptor(pipe_file_descriptors[1]), ReceivePipeFileDescriptor(pipe_file_descriptors[0])))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					EFAULT => panic!("`pipefd` is not valid"),
					EINVAL => panic!("Invalid value in `flags`"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Wraps the standard out pipe.
	///
	/// Normally of very limited value as standard out is nearly always writable.
	#[inline(always)]
	pub fn standard_out() -> Self
	{
		Self(Self::StandardOutFileDescriptor)
	}

	/// Wraps the standard error pipe.
	///
	/// Normally of very limited value as standard error is nearly always writable.
	#[inline(always)]
	pub fn standard_error() -> Self
	{
		Self(Self::StandardErrorFileDescriptor)
	}

	#[inline(always)]
	pub(crate) fn open_fifo<PFD>(fifo_file_path: impl AsRef<Path>, access_flag: c_int, constructor: impl FnOnce(RawFd) -> PFD) -> Result<Option<PFD>, SpecialFileOpenError>
	{
		let fifo_path = CString::new(path_bytes_without_trailing_nul(&fifo_file_path)).unwrap();

		const CommonFlags: c_int = O_CLOEXEC | O_NONBLOCK;

		let result = unsafe { open(fifo_path.as_ptr(), access_flag | CommonFlags) };
		if likely!(result != -1)
		{
			Ok(Some(constructor(result)))
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

					ENXIO => if access_flag == O_WRONLY
					{
						return Ok(None)
					}
					else
					{
						InvalidPath(ExistsButCanNotBeUsed)
					},

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

	/// Uses Linux's `splice()` functionality to move data.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Non-blocking.
	///
	/// `more_is_coming_hint` is used to hint that more data may be sent to `splice_to` soon.
	#[inline(always)]
	pub fn splice_to(&self, splice_from: &impl SpliceSender, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<usize, StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}

		let fd_in = splice_from.as_raw_fd();

		debug_assert_ne!(fd_in, self.0, "Can not splice to self");

		const CommonFlags: c_uint = SPLICE_F_MOVE | SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { splice(fd_in, null_mut(), self.0, null_mut(), maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

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

	/// Uses Linux's `splice()` functionality to move data from an offset within a File.
	///
	/// (To move using the File's current offset, use `SendPipeFileDescriptor::splice_from()` whose `splice_from` can be a File).
	///
	/// Returns the number of bytes transferred and the updated offset.
	///
	/// A successful result returning `0` means end-of-input, unless `maximum_number_of_bytes_to_transfer` was `0`.
	///
	/// Non-blocking.
	///
	/// `more_is_coming_hint` is used to hint that more data may be sent to `splice_to` soon.
	#[inline(always)]
	pub fn splice_to_from_file_offset(&self, splice_from: &File, mut offset: i64, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<(usize, i64), StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok((0, offset))
		}

		let fd_in = splice_from.as_raw_fd();

		debug_assert_ne!(fd_in, self.0, "Can not splice to self");

		const CommonFlags: c_uint = SPLICE_F_MOVE | SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { splice(fd_in, null_mut(), self.0, &mut offset, maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok((result as usize, offset))
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

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
	/// `more_is_coming_hint` is used to hint that more data may be sent to `tee_from` soon.
	#[inline(always)]
	pub fn tee_to(&self, tee_from: &impl SpliceRecipient, maximum_number_of_bytes_to_transfer: usize, more_is_coming_hint: bool) -> Result<usize, StructWriteError>
	{
		if unlikely!(maximum_number_of_bytes_to_transfer == 0)
		{
			return Ok(0)
		}

		let fd_in = tee_from.as_raw_fd();

		debug_assert_ne!(fd_in, self.0, "Can not tee to self");

		const CommonFlags: c_uint = SPLICE_F_NONBLOCK;

		let flags = if unlikely!(more_is_coming_hint)
		{
			CommonFlags | SPLICE_F_MORE
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { tee(fd_in, self.0, maximum_number_of_bytes_to_transfer, flags) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,

					EINTR => Interrupted,

					EINVAL => panic!("`fd_in` and `fd_in` does not refer to a pipe; or `fd_in` and `fd_in` refer to the same pipe"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Copies memory buffers into this pipe.
	///
	/// If `gift` is specified the application may never modify the memory buffers again; gifting existing to make `splice_from()` more efficient, but currently has no advantage as Linux's `splice()` currently does not move memory.
	///
	/// When using `gift` the `memory_buffers` must be aligned.
	///
	/// The maximum number of items in `memory_buffers` is `IOV_MAX` (1024).
	#[inline(always)]
	pub fn vmsplice(&self, memory_buffers: &[iovec], gift_to_kernel: bool) -> Result<usize, StructWriteError>
	{
		const IOV_MAX: usize = 1024;
		debug_assert!(memory_buffers.len() <= IOV_MAX, "too many memory buffers");

		const CommonFlags: c_uint = SPLICE_F_NONBLOCK;

		let flags = if unlikely!(gift_to_kernel)
		{
			CommonFlags | SPLICE_F_GIFT
		}
		else
		{
			CommonFlags
		};

		let result = unsafe { vmsplice(self.0, memory_buffers.as_ptr(), memory_buffers.len() as c_ulong, flags) };

		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::StructWriteError::*;

			Err
			(
				match errno().0
				{
					EAGAIN | ENOMEM => WouldBlock,

					EINTR => Interrupted,

					EBADF => panic!("`fd` is either not valid, or doesn't refer to a pipe"),
					EINVAL => panic!("`nr_segs` is greater than `IOV_MAX`; or memory not aligned if `SPLICE_F_GIFT` set"),

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
