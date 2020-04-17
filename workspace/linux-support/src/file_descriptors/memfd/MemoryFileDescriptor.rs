// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memfd, which wraps a File but supports sealing.
#[derive(Debug)]
pub struct MemoryFileDescriptor(File);

impl IntoRawFd for MemoryFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl FromRawFd for MemoryFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(File::from_raw_fd(fd))
	}
}

impl AsRawFd for MemoryFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl FileDescriptor for MemoryFileDescriptor
{
}

impl Read for MemoryFileDescriptor
{
	#[inline(always)]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{
		self.0.read(buf)
	}

	#[inline(always)]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
	{
		Read::read_vectored(&mut self.0, bufs)
	}

	#[inline(always)]
	unsafe fn initializer(&self) -> Initializer
	{
		self.0.initializer()
	}
}

impl Seek for MemoryFileDescriptor
{
	#[inline(always)]
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>
	{
		self.0.seek(pos)
	}
}

impl Write for MemoryFileDescriptor
{
	#[inline(always)]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>
	{
		self.0.write(buf)
	}

	#[inline(always)]
	fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize>
	{
		Write::write_vectored(&mut self.0, bufs)
	}

	#[inline(always)]
	fn flush(&mut self) -> io::Result<()>
	{
		Ok(())
	}

	#[inline(always)]
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()>
	{
		self.0.write_all(buf)
	}

	#[inline(always)]
	fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()>
	{
		self.0.write_fmt(fmt)
	}
}

impl FileExt for MemoryFileDescriptor
{
	#[inline(always)]
	fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>
	{
		self.0.read_at(buf, offset)
	}

	#[inline(always)]
	fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()>
	{
		self.0.read_exact_at(buf, offset)
	}

	#[inline(always)]
	fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>
	{
		self.0.write_at(buf, offset)
	}

	#[inline(always)]
	fn write_all_at(&self, buf: &[u8], offset: u64) -> io::Result<()>
	{
		self.0.write_all_at(buf, offset)
	}
}

impl ExtendedSeek for MemoryFileDescriptor
{
}

#[allow(deprecated)]
impl AdvisoryWholeFileLocking for MemoryFileDescriptor
{
}

#[allow(deprecated)]
impl PerProcessAdvisoryFileRecordLocking for MemoryFileDescriptor
{
}

impl OpenFileDescriptionAdvisoryFileRecordLocking for MemoryFileDescriptor
{
}

impl CopyFileRange for MemoryFileDescriptor
{
}

impl Leasing for MemoryFileDescriptor
{
}

impl Into<File> for MemoryFileDescriptor
{
	#[inline(always)]
	fn into(self) -> File
	{
		self.0
	}
}

impl Deref for MemoryFileDescriptor
{
	type Target = File;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for MemoryFileDescriptor
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl AsRef<File> for MemoryFileDescriptor
{
	#[inline(always)]
	fn as_ref(&self) -> &File
	{
		&self.0
	}
}

impl AsMut<File> for MemoryFileDescriptor
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut File
	{
		&mut self.0
	}
}

impl Borrow<File> for MemoryFileDescriptor
{
	#[inline(always)]
	fn borrow(&self) -> &File
	{
		&self.0
	}
}

impl BorrowMut<File> for MemoryFileDescriptor
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut File
	{
		&mut self.0
	}
}

impl SpliceRecipient for MemoryFileDescriptor
{
}

impl SpliceSender for MemoryFileDescriptor
{
}

impl VectoredRead for MemoryFileDescriptor
{
	#[inline(always)]
	fn read_vectored(&self, buffers: &[&mut [u8]]) -> io::Result<usize>
	{
		self.0.read_vectored(buffers)
	}
}

impl VectoredWrite for MemoryFileDescriptor
{
	#[inline(always)]
	fn write_vectored(&self, buffers: &[&[u8]]) -> io::Result<usize>
	{
		self.0.write_vectored(buffers)
	}
}

impl SendFile for MemoryFileDescriptor
{
	#[inline(always)]
	fn write_output_from_file<F: AsRef<File>>(&self, from_file: &F, maximum_number_of_bytes_to_transfer: usize) -> Result<usize, StructWriteError>
	{
		self.0.write_output_from_file(from_file, maximum_number_of_bytes_to_transfer)
	}

	#[inline(always)]
	fn write_output_from_file_with_offset<F: AsRef<File>>(&self, from_file: &F, offset: i64, maximum_number_of_bytes_to_transfer: usize) -> Result<(usize, i64), StructWriteError>
	{
		self.0.write_output_from_file_with_offset(from_file, offset, maximum_number_of_bytes_to_transfer)
	}
}

impl MemoryFileDescriptor
{
	/// Opens a memfd.
	///
	/// There are two uses:-
	///
	/// * As an alternative to using `/tmp`, `tmpfs` or `O_TMPFILE` if there is no intention to link a file into the file system;
	/// * To use file sealing (see <http://man7.org/linux/man-pages/man2/fcntl.2.html>); particularly `F_SEAL_FUTURE_WRITE` for a shared memory buffer.
	///
	/// The file can be mmap'd and the file descriptor passed to other processes like any other.
	///
	/// The initial size of the file is set to 0.
	///
	/// Following the call, the file size should be set using `ftruncate()`, or writes made using `write()` or the like.
	///
	/// `non_unique_name_for_debugging_purposes` can be seen under `/proc/self/fd` prefixed with `memfd:`.
	/// It does not have to be unique.
	///
	/// `allow_sealing_operations`:  If true, then the `fcntl()` `F_ADD_SEALS` and `F_GET_SEALS` operations are supported; the initial set of seals is empty.
	/// If not specifed, then the initial set of seals will be `F_SEAL_SEAL`, meaning that no other seals can be set on the file.
	///
	/// `huge_page_size` supports the following:-
	///
	/// * `None`: No huge pages.
	/// * `Some(None)`: Use the system default huge page size.
	/// * `Some(Some(huge_page_size))`: Use the specific `huge_page_size` huge page size.
	///
	/// If the defaults indicate `huge_page_size` `Some(Some(huge_page_size))` is not supported, they will try to find a smaller supported huge page size; if there are not supported huge pages, then the MemoryFileDescriptor will not use huge pages.
	/// If the defaults indicate `huge_page_size` `Some(None)` is not supported, they then the MemoryFileDescriptor will not use huge pages.
	///
	/// The resultant file descriptor will have the close-on-exec flag set (as do all file descriptors created in the super module, `file_descriptors`).
	///
	/// Supported since Linux 3.17.
	/// However, support for `allow_sealing_operations` with `huge_page_size` has only existed since Linux 4.16.
	pub fn open_anonymous_memory_as_file(non_unique_name_for_debugging_purposes: &CStr, allow_sealing_operations: bool, huge_page_size: Option<Option<HugePageSize>>, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<(Self, PageSizeOrHugePageSize), CreationError>
	{
		const MFD_CLOEXEC: u32 = 0x0001;
		const MFD_ALLOW_SEALING: u32 = 0x0002;
		const MFD_HUGETLB: i32 = 0x0004;

		extern "C"
		{
			fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
		}

		let sealing_flags = if allow_sealing_operations
		{
			MFD_ALLOW_SEALING
		}
		else
		{
			0
		};

		let (huge_page_size_flags, page_size) = HugePageSize::mmap_or_memfd_flag_bits_and_page_size(MFD_HUGETLB, huge_page_size, defaults);

		let flags = MFD_CLOEXEC | sealing_flags | huge_page_size_flags as u32;

		let result = unsafe { memfd_create(non_unique_name_for_debugging_purposes.as_ptr() as *const _, flags as u32) };

		if likely!(result == 0)
		{
			return Ok((unsafe { Self::from_raw_fd(result) }, page_size))
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			match errno().0
			{
				EMFILE => Err(PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded),

				ENFILE => Err(SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded),

				ENOMEM => Err(KernelWouldBeOutOfMemory),

				ENOENT => panic!("No queue with this name exists"),

				EINVAL => panic!("The address in name points to invalid memory, or, flags included unknown bits, or, name was too long (The limit is 249 bytes, excluding the
  terminating null byte), or, both  MFD_HUGETLB and MFD_ALLOW_SEALING were specified in flags before Linux 4.16"),

				_ => unreachable!(),
			}
		}
		else
		{
			panic!("Unexpected result {}", result)
		}
	}

	/// Return `Err(())` if permission is denied.
	#[inline(always)]
	pub fn add_file_seals(&self, file_seals: FileSeals) -> Result<(), ()>
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_ADD_SEALS, file_seals.bits) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EPERM => Err(()),

				EINVAL => panic!("This is not a memfd"),

				unexpected @ _ => panic!("Unexpected error `{:?}`", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result from fcntl F_ADD_SEALS of `{}`", result)
		}
	}

	/// Get seals.
	#[inline(always)]
	pub fn get_file_seals(&self) -> FileSeals
	{
		let result = unsafe { fcntl(self.as_raw_fd(), F_GET_SEALS) };
		if likely!(result == 0)
		{
			FileSeals::from_bits_truncate(result)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("This is not a memfd"),

				unexpected @ _ => panic!("Unexpected error `{:?}`", unexpected)
			}
		}
		else
		{
			unreachable!("Unexpected result from fcntl F_ADD_SEALS of `{}`", result)
		}
	}
}
