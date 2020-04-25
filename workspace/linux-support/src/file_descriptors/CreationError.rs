// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during creation of a file descriptor instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum CreationError
{
	/// The per-process limit on the number of open file descriptors would be exceeded.
	///
	/// For epoll creation, this can also be because the per-user limit on the number of epoll instances imposed by `/proc/sys/fs/epoll/max_user_watches` would be exceeded.
	///
	/// For inotify creation, this can also be because the per-user limit on the number of epoll instances imposed by `/proc/sys/fs/inotify/max_user_instances` would be exceeded.
	PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded = EMFILE,

	/// The system-wide limit on the total number of open files would be exceeded.
	///
	/// For POSIX message queues, this can also be caused if a process would exceed `/proc/sys/fs/mqueue/queues_max` and does not have the capability `CAP_SYS_RESOURCE`.
	///
	/// `ENFILE` or sometimes `ENOSPC`.
	SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded = ENFILE,

	/// Kernel would be out of memory.
	///
	/// `ENOMEM` or sometimes `ENOSPC`.
	KernelWouldBeOutOfMemory = ENOMEM,

	/// Occurs for fanotify if the caller lacks the `CAP_SYS_ADMIN` capability.
	///
	/// Occurs for the opening or creation of POSIX message queues, either because of file mode permissions or because the settings for creation (eg maximum message size) are too large (or, on Linux before 3.5, too small).
	///
	/// For memory mappings, an executable mapping is desired for a file opened on a file system mounted noexec or prevented by a file seal.
	///
	/// For process_vm_readv or process_vm_writev, can not access process.
	PermissionDenied = EPERM,

	/// Occurs for pidfd if the pid does not exist.
	///
	/// For process_vm_readv or process_vm_writev, can not access process.
	ProcessForProcessIdentifierDoesNotExist = ESRCH,
}

impl Display for CreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<CreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for CreationError
{
}
