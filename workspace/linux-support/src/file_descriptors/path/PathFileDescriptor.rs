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

impl OnDiskFileDescriptor for PathFileDescriptor
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

	/// Metadata.
	#[inline(always)]
	pub fn metadata_of_self(&self) -> io::Result<Metadata>
	{
		self.use_as_directory(|directory| directory.metadata_of_self())
	}

	/// Metadata.
	#[inline(always)]
	pub fn extended_metadata_of_self(&self, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted) -> io::Result<ExtendedMetadata>
	{
		self.use_as_directory(|directory| directory.extended_metadata_of_self(force_synchronization, extended_metadata_wanted))
	}

	/// Change directory (`cd`).
	///
	/// debug_asserts this is a directory.
	#[inline(always)]
	pub fn change_current_working_directory_to_self(&self) -> io::Result<()>
	{
		debug_assert!(self.is_directory, "is not a directory");

		self.use_as_directory(|directory| directory.change_current_working_directory_to_self())
	}

	/// Only a process with the `CAP_CHOWN` capability may change the owner of a file.
	///
	/// The owner of a file may change the group of the file to any group of which that owner is a member.
	///
	/// A process with the `CAP_CHOWN` capability may change the group arbitrarily.
	///
	/// When the owner or group of an executable file is changed by an unprivileged user the `S_ISUID` (suid) and `S_ISGID` (sgid) mode bits are cleared.
	/// In case of a  non-group-executable file (ie, one for which the `S_IXGRP` bit is not set) the `S_ISGID` bit indicates mandatory locking (which is not usually used by Linux in any event), and is not cleared.
	///
	/// When the owner or group of an executable file is changed (by any user), all capability sets for the path are cleared.
	///
	/// If `owner` is `None` no changes are made.
	/// If `group` is `None` no changes are made.
	///
	/// Since Linux 2.6.39.
	#[inline(always)]
	pub fn change_ownership_of_self(&self, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>) -> io::Result<()>
	{
		self.use_as_directory(|directory| directory.change_ownership_of_self(owner, group, false))
	}

	/// Name to file handle.
	///
	/// Not supported by `/sys` and /proc` and possibly other file systems.
	#[inline(always)]
	pub fn name_to_handle_for_self(&self) -> io::Result<LinuxFileHandle>
	{
		self.use_as_directory(|directory| directory.name_to_handle_for_self())
	}

	/// Create a hard link.
	///
	/// If `to` and `to_path` exists, it will *NOT* be overwritten.
	///
	/// `to_path` can be absolute.
	///
	/// Process must have the `CAP_DAC_READ_SEARCH` capability.
	#[inline(always)]
	pub fn make_hard_link_for_self(&self, to: &DirectoryFileDescriptor, to_path: &CStr) -> io::Result<()>
	{
		debug_assert!(!self.is_directory, "is a directory");

		self.use_as_directory(|directory| directory.make_hard_link_for_self(to, to_path))
	}

	/// Execute a command with a new environment but keep the current process identifier, any file descriptors not set to close-on-exec, etc.
	///
	/// This is useful if one has execute permission but NOT read permission.
	///
	/// `arguments[0]` should ideally be the same path as `self` represents.
	///
	/// debug_asserts this is not a directory.
	#[inline(always)]
	pub fn execve(&self, arguments: &NulTerminatedCStringArray, environment: &Environment) -> io::Result<!>
	{
		debug_assert!(!self.is_directory, "is a directory");

		self.use_as_directory(|directory| directory.execve_for_self(false, arguments, environment))
	}

	#[inline(always)]
	fn use_as_directory<R>(&self, callback: impl FnOnce(&DirectoryFileDescriptor) -> R) -> R
	{
		let directory = DirectoryFileDescriptor(self.raw_fd);
		let result = callback(&directory);
		forget(directory);
		result
	}
}
