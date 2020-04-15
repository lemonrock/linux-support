// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A directory file descriptor.
///
/// Defaults to the current working directory (and updates as that changes).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectoryFileDescriptor(RawFd);

impl Drop for DirectoryFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0 != AT_FDCWD
		{
			self.0.close()
		}
	}
}

impl AsRawFd for DirectoryFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl AsRawFdExt for DirectoryFileDescriptor
{
}

impl IntoRawFd for DirectoryFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for DirectoryFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

// TODO: O_PATH

impl DirectoryFileDescriptor
{
	/// A special file descriptor that always refers to the current working directory.
	pub const AlwaysCurrentWorkingDirectory: Self = Self(AT_FDCWD);

	// TODO: Detect BLOCK, CHARACTER, FIFO, SOCKET, ?DIRECTORY and return a suitable type.
	// NOTE: Not much we can do with SOCKET

	/// `disable_linux_page_cache` is used to specify `O_DIRECT`.
	///
	/// Unlike a regular open, the file descriptor is set to be close-on-exec.
	///
	/// Uses `openat()` under the covers.
	#[inline(always)]
	pub fn open<'a>(&self, file_open_kind: FileOpenKind<'a>, disable_linux_page_cache: bool) -> io::Result<File>
	{
		let (o_flags, mode, path) = file_open_kind.o_flags_mode_and_path();

		let o_flags = o_flags | O_CLOEXEC | if disable_linux_page_cache
		{
			O_DIRECT
		}
		else
		{
			0
		};

		let result = unsafe { openat(self.as_raw_fd(), path, o_flags, mode) };
		if likely!(result >= 0)
		{
			Ok(unsafe { File::from_raw_fd(result) })
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("result of openat() was unexpected value {}", result)
		}
	}

	/// `disable_linux_page_cache` is used to specify `O_DIRECT`.
	///
	/// Unlike a regular open, the file descriptor is set to be close-on-exec.
	///
	/// Uses `openat2()` under the covers.
	///
	/// Since Linux 5.6.
	#[inline(always)]
	pub fn open2<'a>(&self, file_open_kind: FileOpenKind<'a>, disable_linux_page_cache: bool, path_resolution: PathResolution) -> io::Result<File>
	{
		let (o_flags, mode, path) = file_open_kind.o_flags_mode_and_path();

		let o_flags = o_flags | O_CLOEXEC | if disable_linux_page_cache
		{
			O_DIRECT
		}
		else
		{
			0
		};

		let mut how = open_how
		{
			flags: o_flags as u64,
			mode: mode as u64,
			resolve: path_resolution.bits,
		};
		let result = openat2(self.as_raw_fd(), path, &mut how, size_of::<open_how>());
		if likely!(result >= 0)
		{
			Ok(unsafe { File::from_raw_fd(result) })
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("result of openat2() was unexpected value {}", result)
		}
	}

//	openat2
//	fchownat
//	fchmodat
//	mkdirat
//	fstatat
//	unlinkat
//	linkat
//	http://man7.org/linux/man-pages/man2/open_by_handle_at.2.html


	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_character_device(&self, path: &CStr, mode: mode_t, device: CharacterDevice) -> io::Result<()>
	{
		self.make_node_at(path, mode, S_IFCHR, device.into())
	}

	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_block_device(&self, path: &CStr, mode: mode_t, device: BlockDevice) -> io::Result<()>
	{
		self.make_node_at(path, mode, S_IFBLK, device.into())
	}

	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_unix_domain_socket(&self, path: &CStr, mode: mode_t) -> io::Result<()>
	{
		self.make_node_at(path, mode, S_IFSOCK, 0u64)
	}

	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_fifo(&self, path: &CStr, mode: mode_t) -> io::Result<()>
	{
		self.make_node_at(path, mode, S_IFIFO, 0u64)
	}

	#[inline(always)]
	fn make_node_at(&self, path: &CStr, mode: mode_t, top_bits: mode_t, device: dev_t) -> io::Result<()>
	{
		let result = unsafe { mknodat(self.0, path.as_ptr(), Self::mask_mode(mode, top_bits), device) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("mknodat() returned unexpected result {}", result)
		}
	}

	#[inline(always)]
	const fn mask_mode(mode: mode_t, top_bits: mode_t) -> mode_t
	{
		(mode & 0xFFF) | top_bits
	}
}
