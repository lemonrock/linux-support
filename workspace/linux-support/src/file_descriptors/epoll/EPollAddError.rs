// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Error for adding to an epoll instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EPollAddError
{
	/// There was insufficient kernel memory to complete the operation.
	ThereWasInsufficientKernelMemory,

	/// The limit imposed by `/proc/sys/fs/epoll/max_user_watches` would be exceeded.
	LimitOnWatchesWouldBeExceeded,

	/// `EINTR` or `EAGAIN` in io_uring.
	TryAgain,
}

impl Display for EPollAddError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<EPollAddError as Debug>::fmt(self, f)
	}
}

impl error::Error for EPollAddError
{
}
