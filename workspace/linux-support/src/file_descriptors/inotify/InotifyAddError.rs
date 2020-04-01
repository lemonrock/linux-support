// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during binding of a socket instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InotifyAddError
{
	/// Permission denied.
	PermissionDenied,

	/// Kernel would be out of memory.
	KernelWouldBeOutOfMemory,

	/// A directory component in `pathname` does not exist or is a dangling symbolic link.
	FilePathInvalid,

	/// The limit on the maximum number of watches would be exceeded, or the kernel is lacking a resource required.
	///
	/// The maximum number of open (added) watch descriptors is specified in `/proc/sys/fs/inotify/max_user_watches`.
	MaximumNumberOfWatchesWouldBeExceeded,
}

impl Display for InotifyAddError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<InotifyAddError as Debug>::fmt(self, f)
	}
}

impl error::Error for InotifyAddError
{
}
