// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a path in a file system.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFileDescriptor(RawFd);

impl Drop for PathFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for PathFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl AsRawFdExt for PathFileDescriptor
{
}

impl IntoRawFd for PathFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for PathFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl PathFileDescriptor
{

}

/*
The following operations can be performed on the resulting
              file descriptor:

              *  close(2).

              *  fchdir(2), if the file descriptor refers to a directory
                 (since Linux 3.5).

              *  fstat(2) (since Linux 3.6).

              *  fstatfs(2) (since Linux 3.12).

              *  Duplicating the file descriptor (dup(2), fcntl(2) F_DUPFD,
                 etc.).

              *  Getting and setting file descriptor flags (fcntl(2) F_GETFD
                 and F_SETFD).

              *  Retrieving open file status flags using the fcntl(2)
                 F_GETFL operation: the returned flags will include the bit
                 O_PATH.

              *  Passing the file descriptor as the dirfd argument of
                 openat() and the other "*at()" system calls.  This includes
                 linkat(2) with AT_EMPTY_PATH (or via procfs using
                 AT_SYMLINK_FOLLOW) even if the file is not a directory.

              *  Passing the file descriptor to another process via a UNIX
                 domain socket (see SCM_RIGHTS in unix(7)).

              When O_PATH is specified in flags, flag bits other than
              O_CLOEXEC, O_DIRECTORY, and O_NOFOLLOW are ignored.
*/
