// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A directory file descriptor.
///
/// Defaults to the current working directory (and updates as that changes).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectoryFileDescriptor(pub(crate) RawFd);

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

impl FileDescriptor for DirectoryFileDescriptor
{
}

impl OnDiskFileDescriptor for DirectoryFileDescriptor
{
}

impl DirectoryFileDescriptor
{
	/// A special file descriptor that always refers to the current working directory.
	pub const AlwaysCurrentWorkingDirectory: Self = Self(AT_FDCWD);

	/// A new directory file descriptor.
	///
	/// Unlike a regular open, the file descriptor is set to be close-on-exec.
	///
	/// Uses `open()` under the covers.
	#[inline(always)]
	pub fn new(path: &CStr) -> io::Result<Self>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { open(path.as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from open()", result)
		}
	}

	/// A new directory file descriptor.
	///
	/// Unlike a regular open, the file descriptor is set to be close-on-exec.
	///
	/// Uses `openat()` under the covers.
	///
	/// `path` can be absolute.
	#[inline(always)]
	pub fn new_relative_to_self(&self, path: &CStr) -> io::Result<Self>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { openat(self.as_raw_fd(), path.as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
		if likely!(result >= 0)
		{
			Ok(Self(result))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from openat()", result)
		}
	}

	/// A new directory file descriptor.
	///
	/// Unlike a regular open, the file descriptor is set to be close-on-exec.
	///
	/// Uses `openat2()` under the covers.
	///
	/// `path` can be absolute.
	#[inline(always)]
	pub fn new_relative_to_self2(&self, path: &CStr, path_resolution: PathResolution) -> io::Result<Self>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let mut how = open_how
		{
			flags: (O_RDONLY | O_DIRECTORY | O_CLOEXEC) as u64,
			mode: 0,
			resolve: path_resolution.bits,
		};
		let result = openat2(self.as_raw_fd(), path.as_ptr(), &mut how, size_of::<open_how>());
		if likely!(result >= 0)
		{
			Ok(Self(result as RawFd))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from openat2()", result)
		}
	}

	/// TODO: Detect BLOCK, CHARACTER, FIFO, SOCKET, ?DIRECTORY and return a suitable type.
	/// NOTE: Not much we can do with SOCKET.
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
			Ok(unsafe { File::from_raw_fd(result as RawFd) })
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

	/// Name to file handle.
	///
	/// Not supported by `/sys` and `/proc` and possibly other file systems.
	#[inline(always)]
	pub fn name_to_handle(&self, path: &CStr, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<LinuxFileHandle>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.name_to_handle_internal(Self::non_empty_path(path), do_not_dereference_path_if_it_is_a_symlink, 0)
	}

	/// Name to file handle for self.
	///
	/// Not supported by `/sys` and /proc` and possibly other file systems.
	#[inline(always)]
	pub fn name_to_handle_for_self(&self) -> io::Result<LinuxFileHandle>
	{
		self.name_to_handle_internal(Self::empty_path(), true, AT_EMPTY_PATH)
	}

	#[inline(always)]
	fn name_to_handle_internal(&self, path: NonNull<c_char>, do_not_dereference_path_if_it_is_a_symlink: bool, flags: i32) -> io::Result<LinuxFileHandle>
	{
		let mut file_handle = file_handle::new();

		#[allow(deprecated)]
		let mut mount_id = unsafe { uninitialized() };
		let flags = flags | if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			0
		}
		else
		{
			AT_SYMLINK_FOLLOW
		};

		let result = unsafe { name_to_handle_at(self.as_raw_fd(), path.as_ptr(), &mut file_handle, &mut mount_id, flags) };
		if likely!(result == 0)
		{
			Ok
			(
				LinuxFileHandle
				{
					file_system_mount_identifier: FileSystemMountIdentifier(mount_id),
					file_handle
				}
			)
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("result {} was unexpected for name_to_handle_at()", result)
		}
	}

	/// Iterate over entries in directory.
	#[inline(always)]
	pub fn iterate(&self) -> DirectoryEntryIterator
	{
		DirectoryEntryIterator::new(self)
	}

	/// Freezing a file system ensures that it is in a stable state to back up or duplicate at a byte level.
	///
	/// Writes will be blocked.
	///
	/// `drop()` the resulting `FrozenFileSystem` to thaw the file system.
	///
	/// Read-only operations are still permitted on a frozen file system.
	///
	/// `self` should be a mount point.
	pub fn freeze_file_system<'a>(&'a self) -> io::Result<FrozenFileSystem<'a>>
	{
		let result = unsafe { ioctl(self.as_raw_fd(), FIFREEZE, 0) };
		if likely!(result == 0)
		{
			Ok(FrozenFileSystem(self))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from ioctl()", result)
		}
	}

	#[inline(always)]
	pub(crate) fn thaw_file_system(&self) -> io::Result<()>
	{
		let result = unsafe { ioctl(self.as_raw_fd(), FITHAW, 0) };
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
			unreachable!("Unexpected result {} from ioctl()", result)
		}
	}

	/// Do a TRIM to discard extents on a SSD or thin storage, effectively defragmenting them.
	///
	/// TRIMs should be done no more than once a week on most SSDs to preserve their life.
	///
	/// Logic derived from `fstrim` in `util-linux`.
	///
	/// Returns `Ok(true)` if trim succeeded and `Ok(false)` if the file system does not support it in some way.
	///
	/// `self` should be a mount point.
	///
	/// # `physical_range`
	///
	/// This is the range within the file system to search.
	///
	/// It can be set to `0 ..= u64::MAX`.
	///
	/// # `minimum_size_of_extend_to_discard`
	/// Minimum contiguous free range to discard, in bytes. (This value is internally rounded up to a multiple of the filesystem block size.)
	/// Free ranges smaller than this will be ignored; this is a device-specific minimum.
	///
	/// By increasing this value, the `trim()` operation will complete more quickly for filesystems with badly fragmented free space, although not all blocks will be discarded.
	/// A value of zero will discard every free block.
	#[inline(always)]
	pub fn trim_file_system(&self, physical_range: RangeInclusive<u64>, minimum_size_of_extend_to_discard: u64) -> io::Result<bool>
	{
		let (start, end) = physical_range.into_inner();
		let length = end - start + 1;
		let trim = fstrim_range
		{
			start,
			len: length,
			minlin: minimum_size_of_extend_to_discard,
		};

		let result = unsafe { ioctl(self.as_raw_fd(), FITRIM, &trim) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF | ENOTTY | EOPNOTSUPP => Ok(false),
				_ => Err(io::Error::last_os_error()),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from ioctl()", result)
		}
	}

	/// Change directory (`cd`).
	#[inline(always)]
	pub fn change_current_working_directory_to_self(&self) -> io::Result<()>
	{
		let result = unsafe { fchdir(self.as_raw_fd()) };
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
			unreachable!("Unexpected result {} from fchdir()", result)
		}
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
	/// `path` can be absolute.
	#[inline(always)]
	pub fn change_ownership(&self, path: &CStr, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.change_ownership_internal(Self::non_empty_path(path), owner, group, do_not_dereference_path_if_it_is_a_symlink, 0)
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
	pub fn change_ownership_of_self(&self, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		self.change_ownership_internal(Self::empty_path(), owner, group, do_not_dereference_path_if_it_is_a_symlink, AT_EMPTY_PATH)
	}

	#[inline(always)]
	fn change_ownership_internal(&self, path: NonNull<c_char>, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>, do_not_dereference_path_if_it_is_a_symlink: bool, flags: i32) -> io::Result<()>
	{
		let flags = flags | if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		};

		let (uid, gid) = match (owner, group)
		{
			(Some(owner), Some(group)) => (owner.into(), group.into()),
			(Some(owner), None) => (owner.into(), -1i32 as u32),
			(None, Some(group)) => (-1i32 as u32, group.into()),
			(None, None) => return Ok(()),
		};

		let result = unsafe { fchownat(self.as_raw_fd(), path.as_ptr(), uid, gid, flags) };
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
			unreachable!("result of openat2() was unexpected value {}", result)
		}
	}

	/// The effective UID of the calling process must match the owner of the file, or the process have the `CAP_FOWNER` capability.
	///
	/// If effective UID of the calling process doesn't match and the process doesn't have the `CAP_FOWNER` capability, and the group of the file does not match the effective group ID of the process or one of its supplementary group IDs, the `S_ISGID` bit will be turned off, but this will not cause an error to be returned.
	///
	/// If the process does not have the `CAP_FSETID` capability, as a security measure, depending on the filesystem, the set-user-ID and set-group-ID execution bits may be turned off if a file is written.
	///
	/// On some filesystems, only the superuser can set the sticky bit, which may have a special meaning.
	/// On NFS filesystems, restricting the permissions will immediately influence already open files, because the access control is done on  the server, but open files are maintained by the client.
	/// Widening the permissions may be delayed for other clients if attribute caching is enabled on them.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn change_mode(&self, path: &CStr, access_permissions: AccessPermissions, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let flags = if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		};
		let result = unsafe { fchmodat(self.as_raw_fd(), path.as_ptr(), Self::mask_mode(access_permissions), flags) };
		if likely!(result >= 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from fchmodat()", result)
		}
	}

	/// To set both file timestamps to the current time (`last_access_time` and `last_modification_time` are both `TimestampUpdate::Now`):-
	///
	/// * the caller must have write access to the file; and
	/// * either the caller's effective user ID must match the owner of the file; or
	/// * the caller must have appropriate privileges.
	///
	/// To make any change other than setting both timestamps to the current time:-
	///
	/// * either the caller's effective user ID must match the owner of the file; or
	/// * the caller must have appropriate privileges.
	///
	/// Even if `last_access_time` and `last_modification_time` are both set to `TimestampUpdate::DoNotChange` errors can occur.
	///
	/// `last_access_time` is also known as `atime`.
	/// `last_modification_time` is also known as `mtime`.
	#[inline(always)]
	pub fn change_timestamps(&self, path: &CStr, last_access_time: &TimestampUpdate, last_modification_time: &TimestampUpdate, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let times =
		[
			last_access_time.to_timespec(),
			last_modification_time.to_timespec(),
		];

		let flags = if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		};

		let result = unsafe { utimensat(self.as_raw_fd(), path.as_ptr(), times.as_ptr(), flags) };
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
			unreachable!("utimensat() returned unexpected result {}", result)
		}
	}

	/// Is this `path` accessible?
	///
	/// `path` can be absolute.
	#[inline(always)]
	pub fn accessible_to_real_user_and_group(&self, path: &CStr, accessibility: Accessibility, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<bool>
	{
		self.accessible_internal(path, accessibility, do_not_dereference_path_if_it_is_a_symlink, 0)
	}

	/// Is this `path` accessible?
	///
	/// `path` can be absolute.
	#[inline(always)]
	pub fn accessible_to_effective_user_and_group(&self, path: &CStr, accessibility: Accessibility, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<bool>
	{
		self.accessible_internal(path, accessibility, do_not_dereference_path_if_it_is_a_symlink, AT_EACCESS)
	}

	#[inline(always)]
	fn accessible_internal(&self, path: &CStr, accessibility: Accessibility, do_not_dereference_path_if_it_is_a_symlink: bool, flags: i32) -> io::Result<bool>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let flags = flags | if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		};

		let result = unsafe { faccessat(self.as_raw_fd(), path.as_ptr(), accessibility.bits as i32, flags) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EACCES  | ENOENT | ENOTDIR | EROFS => Ok(false),
				other @ _ => Err(io::Error::from_raw_os_error(other)),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from fchmodat()", result)
		}
	}

	/// Create a hard link.
	///
	/// If `to` and `to_path` exists, it will *NOT* be overwritten.
	///
	/// `from_path` and `to_path` can be absolute.
	#[inline(always)]
	pub fn make_hard_link(&self, from_path: &CStr, to: &Self, to_path: &CStr, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		self.make_hard_link_internal(Self::non_empty_path(from_path), to, to_path, do_not_dereference_path_if_it_is_a_symlink)
	}

	#[inline(always)]
	pub(crate) fn make_hard_link_for_self(&self, to: &Self, to_path: &CStr) -> io::Result<()>
	{
		self.make_hard_link_internal(Self::empty_path(), to, to_path, true)
	}

	#[inline(always)]
	fn make_hard_link_internal(&self, from_path: NonNull<c_char>, to: &Self, to_path: &CStr, do_not_dereference_path_if_it_is_a_symlink: bool) -> io::Result<()>
	{
		debug_assert!(!to_path.to_bytes().is_empty(), "Empty to_path is not permitted");

		let flags = if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			0
		}
		else
		{
			AT_SYMLINK_FOLLOW
		};

		let result = unsafe { linkat(self.as_raw_fd(), from_path.as_ptr(), to.as_raw_fd(), to_path.as_ptr(), flags) };
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
			unreachable!("unlinkat() returned unexpected result {}", result)
		}
	}

	/// Create a symbolic link (symlink) at `path` pointing to `target` (which need not be a valid path; ultimately, it's a C string stored as the text contents of a file).
	///
	/// `path` can be absolute.
	#[inline(always)]
	pub fn make_symbolic_link(&self, path: &CStr, target: &CStr) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { symlinkat(target.as_ptr(), self.as_raw_fd(), path.as_ptr()) };
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
			unreachable!("unlinkat() returned unexpected result {}", result)
		}
	}

	/// Create a hard link.
	///
	/// If `to` and `to_path` exists, it will *NOT* be overwritten.
	///
	/// `from_path` and `to_path` can be absolute.
	#[inline(always)]
	pub fn rename(&self, from_path: &CStr, to: &Self, to_path: &CStr, rename_flags: RenameFlags) -> io::Result<()>
	{
		debug_assert!(!from_path.to_bytes().is_empty(), "Empty from_path is not permitted");
		debug_assert!(!to_path.to_bytes().is_empty(), "Empty to_path is not permitted");

		let result = renameat2(self.as_raw_fd(), from_path.as_ptr(), to.as_raw_fd(), to_path.as_ptr(), rename_flags as i32);
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
			unreachable!("unlinkat() returned unexpected result {}", result)
		}
	}

	/// Execute a command with a new environment but keep the current process identifier, any file descriptors not set to close-on-exec, etc.
	///
	/// `arguments[0]` should ideally be the same as `path`.
	#[inline(always)]
	pub fn execve(&self, path: &CStr, do_not_dereference_path_if_it_is_a_symlink: bool, arguments: &NulTerminatedCStringArray, environment: &Environment) -> io::Result<!>
	{
		self.execve_internal(Self::non_empty_path(path), do_not_dereference_path_if_it_is_a_symlink, arguments, environment, 0)
	}

	#[inline(always)]
	pub(crate) fn execve_for_self(&self, do_not_dereference_path_if_it_is_a_symlink: bool, arguments: &NulTerminatedCStringArray, environment: &Environment) -> io::Result<!>
	{
		self.execve_internal(Self::empty_path(), do_not_dereference_path_if_it_is_a_symlink, arguments, environment, AT_EMPTY_PATH)
	}

	/// Execute a command with a new environment but keep the current process identifier, any file descriptors not set to close-on-exec, etc.
	#[inline(always)]
	fn execve_internal(&self, path: NonNull<c_char>, do_not_dereference_path_if_it_is_a_symlink: bool, arguments: &NulTerminatedCStringArray, environment: &Environment, flags: i32) -> io::Result<!>
	{
		let environment = environment.to_environment_c_string_array();

		let flags = flags | if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		};
		let result = unsafe { execveat(self.as_raw_fd(), path.as_ptr(), arguments.as_array_of_pointers(), environment.as_array_of_pointers(), flags) };
		if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("execveat returned a value of `{}`", result)
		}
	}

	/// Extended metadata.
	///
	/// `do_not_automount_basename_of_path` uses the flag `AT_NO_AUTOMOUNT`.
	/// `force_synchronization` can be made to control synchronization so that timestamps and the like are accurate; if `None`, it mirrors whatever `self.metadata()` might do.
	#[inline(always)]
	pub fn extended_metadata(&self, path: &CStr, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool) -> io::Result<ExtendedMetadata>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.extended_metadata_internal(Self::non_empty_path(path), force_synchronization, extended_metadata_wanted, do_not_dereference_path_if_it_is_a_symlink, do_not_automount_basename_of_path, 0)
	}

	/// Metadata.
	#[inline(always)]
	pub fn extended_metadata_of_self(&self, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted) -> io::Result<ExtendedMetadata>
	{
		self.extended_metadata_internal(Self::empty_path(), force_synchronization, extended_metadata_wanted, false, false, AT_EMPTY_PATH)
	}

	#[inline(always)]
	fn extended_metadata_internal(&self, path: NonNull<c_char>, force_synchronization: Option<bool>, extended_metadata_wanted: ExtendedMetadataWanted, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool, flags: i32) -> io::Result<ExtendedMetadata>
	{
		let flags = flags
		| match force_synchronization
		{
			None => AT_STATX_SYNC_AS_STAT,
			Some(true) => AT_STATX_FORCE_SYNC,
			Some(false) => AT_STATX_DONT_SYNC,
		}
		| if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		}
		| if unlikely!(do_not_automount_basename_of_path)
		{
			AT_NO_AUTOMOUNT
		}
		else
		{
			0
		};

		#[allow(deprecated)]
		let mut statx: statx = unsafe { uninitialized() };

		let result = statx_(self.as_raw_fd(), path.as_ptr(), flags as u32, extended_metadata_wanted.bits, &mut statx);
		if likely!(result == 0)
		{
			statx.zero_padding();
			Ok(ExtendedMetadata(statx))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("unlinkat() returned unexpected result {}", result)
		}
	}

	/// Metadata.
	///
	/// `do_not_automount_basename_of_path` uses the flag `AT_NO_AUTOMOUNT`.
	#[inline(always)]
	pub fn metadata(&self, path: &CStr, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool) -> io::Result<Metadata>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.metadata_internal(Self::non_empty_path(path), do_not_dereference_path_if_it_is_a_symlink, do_not_automount_basename_of_path, 0)
	}

	/// Metadata.
	#[inline(always)]
	pub fn metadata_of_self(&self) -> io::Result<Metadata>
	{
		self.metadata_internal(Self::empty_path(), false, false, AT_EMPTY_PATH)
	}

	#[inline(always)]
	fn metadata_internal(&self, path: NonNull<c_char>, do_not_dereference_path_if_it_is_a_symlink: bool, do_not_automount_basename_of_path: bool, flags: i32) -> io::Result<Metadata>
	{
		let flags = flags | if unlikely!(do_not_dereference_path_if_it_is_a_symlink)
		{
			AT_SYMLINK_NOFOLLOW
		}
		else
		{
			0
		} | if unlikely!(do_not_automount_basename_of_path)
		{
			AT_NO_AUTOMOUNT
		}
		else
		{
			0
		};

		#[allow(deprecated)]
		let mut buffer = unsafe { uninitialized() };
		let result = unsafe { fstatat(self.as_raw_fd(), path.as_ptr(), &mut buffer, flags) };
		if likely!(result == 0)
		{
			Ok(Metadata(buffer))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("fstatat() returned unexpected result {}", result)
		}
	}

	/// `path` can be absolute.
	///
	/// Like `rm` or `unlink`.
	#[inline(always)]
	pub fn remove(&self, path: &CStr) -> io::Result<()>
	{
		self.unlink(path, 0)
	}

	/// `path` can be absolute.
	///
	/// Like `rmdir`.
	#[inline(always)]
	pub fn remove_directory_if_empty(&self, path: &CStr) -> io::Result<()>
	{
		self.unlink(path, AT_REMOVEDIR)
	}

	#[inline(always)]
	fn unlink(&self, path: &CStr, flags: i32) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { unlinkat(self.as_raw_fd(), path.as_ptr(), flags) };
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
			unreachable!("unlinkat() returned unexpected result {}", result)
		}
	}

	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_directory(&self, path: &CStr, access_permissions: AccessPermissions) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { mkdirat(self.0, path.as_ptr(), Self::mask_mode(access_permissions)) };
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
			unreachable!("mkdirat() returned unexpected result {}", result)
		}
	}

	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_character_device(&self, path: &CStr, access_permissions: AccessPermissions, device: CharacterDevice) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.make_node_at(path, access_permissions, S_IFCHR, device.into())
	}

	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_block_device(&self, path: &CStr, access_permissions: AccessPermissions, device: BlockDevice) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.make_node_at(path, access_permissions, S_IFBLK, device.into())
	}

	/// The caller must have the capability `CAP_MKNOD`.
	///
	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_unix_domain_socket(&self, path: &CStr, access_permissions: AccessPermissions) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.make_node_at(path, access_permissions, S_IFSOCK, 0u64)
	}

	/// `path` can be absolute.
	///
	/// Only the bottom 12 bits of mode are kept and the call masks them using the process' current `umask()`.
	#[inline(always)]
	pub fn make_fifo(&self, path: &CStr, access_permissions: AccessPermissions) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		self.make_node_at(path, access_permissions, S_IFIFO, 0u64)
	}

	#[inline(always)]
	fn make_node_at(&self, path: &CStr, access_permissions: AccessPermissions, top_bits: mode_t, device: dev_t) -> io::Result<()>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		let result = unsafe { mknodat(self.0, path.as_ptr(), Self::mask_mode_and_apply(access_permissions, top_bits), device) };
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
	const fn mask_mode_and_apply(access_permissions: AccessPermissions, top_bits: mode_t) -> mode_t
	{
		Self::mask_mode(access_permissions) | top_bits
	}

	#[inline(always)]
	pub(crate) const fn mask_mode(access_permissions: AccessPermissions) -> mode_t
	{
		access_permissions.0 & 0xFFF
	}

	#[inline(always)]
	fn non_empty_path(path: &CStr) -> NonNull<c_char>
	{
		debug_assert!(!path.to_bytes().is_empty(), "Empty path is not permitted");

		unsafe { NonNull::new_unchecked(path.as_ptr() as *mut _) }
	}

	#[inline(always)]
	fn empty_path() -> NonNull<c_char>
	{
		const EmptyPath: &'static [u8] = b"\0";
		unsafe { NonNull::new_unchecked(EmptyPath.as_ptr() as *const u8 as *const c_char as *mut _) }
	}
}
