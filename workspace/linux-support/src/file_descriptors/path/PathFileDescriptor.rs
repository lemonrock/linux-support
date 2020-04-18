// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a path in a file system.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFileDescriptor
{
	raw_fd: RawFd,
	is_directory: bool,
}

impl Drop for PathFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.raw_fd.close()
	}
}

impl AsRawFd for PathFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.raw_fd
	}
}

impl IntoRawFd for PathFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.raw_fd
	}
}

impl FromRawFd for PathFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		let o_flags = Self::get_o_flags_raw_fd(fd);
		debug_assert_ne!(o_flags & O_PATH, 0, "Not a path");

		Self
		{
			raw_fd: fd,
			is_directory: o_flags & O_DIRECTORY != 0,
		}
	}
}

impl FileDescriptor for PathFileDescriptor
{
}

impl PathFileDescriptor
{
	/// Open a new path file descriptor.
	#[inline(always)]
	pub fn open(path: &CStr, is_directory: bool, no_follow: bool) -> io::Result<Self>
	{
		let flags = O_PATH | O_CLOEXEC | if is_directory
		{
			O_DIRECTORY
		}
		else
		{
			0
		}
		| if no_follow
		{
			O_NOFOLLOW
		}
		else
		{
			0
		};

		let result = unsafe { open(path.as_ptr(), flags) };
		if likely!(result >= 0)
		{
			Ok
			(
				Self
				{
					raw_fd: result,
					is_directory,
				}
			)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("open() returned unexpected result {}", result)
		}
	}

	/// Change directory (`cd`).
	///
	/// Returns `Ok(false)` if this is not a directory.
	#[inline(always)]
	pub fn change_current_working_directory_to_self(&self) -> io::Result<bool>
	{
		if unlikely!(self.is_directory)
		{
			return Ok(false)
		}
		let result = unsafe { fchdir(self.as_raw_fd()) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from fchdir()", result)
		}
	}

	/// Metadata.
	#[inline(always)]
	pub fn metadata_of_self(&self) -> io::Result<Metadata>
	{
		let directory = DirectoryFileDescriptor(self.raw_fd);
		let result = directory.metadata_of_self();
		forget(directory);
		result
	}
}

// TODO: execveat() for Directory.

///// Represents a file descriptor backed by real storage.
//pub trait OnDiskFileDescriptor: FileDescriptor
//{
//
//}

/*
The following operations can be performed on the resulting
              file descriptor:

              *  fstatfs(2) (since Linux 3.12).

              *  Passing the file descriptor as the dirfd argument of
                 openat() and the other "*at()" system calls.  This includes
                 linkat(2) with AT_EMPTY_PATH (or via procfs using
                 AT_SYMLINK_FOLLOW) even if the file is not a directory.

FIND ALL `*at()` functions that can take an empty path.
    * statx()
    * fstatat()
    * renameat2()
    * name_to_handle_at()
    * fchownat()
    * linkat()  - needs CAP_DAC_READ_SEARCH capability.
    * execveat()
*/
