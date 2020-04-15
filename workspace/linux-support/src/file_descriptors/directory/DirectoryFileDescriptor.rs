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
	/// `path` can be absolute.
	#[inline(always)]
	pub fn change_ownership(&self, path: &CStr, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>) -> io::Result<()>
	{
		self.change_ownership_internal(unsafe { NonNull::new_unchecked(path.as_ptr() as *mut _) }, owner, group, 0)
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
		const EmptyPath: &'static [u8] = b"\0";
		self.change_ownership_internal(unsafe { NonNull::new_unchecked(EmptyPath.as_ptr() as *const u8 as *const c_char as *mut _) }, owner, group, AT_EMPTY_PATH)
	}

	#[inline(always)]
	fn change_ownership_internal(&self, path: NonNull<c_char>, owner: Option<UserIdentifier>, group: Option<GroupIdentifier>, flags: i32) -> io::Result<()>
	{
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
	pub fn change_mode(&self, path: &CStr, mode: mode_t) -> io::Result<()>
	{
		const FlagsAreUselessOnLinux: i32 = 0;
		let result = unsafe { fchmodat(self.as_raw_fd(), path.as_ptr(), Self::mask_mode(mode), FlagsAreUselessOnLinux) };
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
		let result = unsafe { mknodat(self.0, path.as_ptr(), Self::mask_mode_and_apply(mode, top_bits), device) };
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
	const fn mask_mode_and_apply(mode: mode_t, top_bits: mode_t) -> mode_t
	{
		Self::mask_mode(mode) | top_bits
	}

	#[inline(always)]
	const fn mask_mode(mode: mode_t) -> mode_t
	{
		mode & 0xFFF
	}
}
