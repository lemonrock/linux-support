// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Error for modifying an epoll instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EPollModifyError
{
	/// There was insufficient kernel memory to complete the operation.
	ThereWasInsufficientKernelMemory,
	
	/// `EINTR` or `EAGAIN` in io_uring.
	TryAgain,
}

impl Display for EPollModifyError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<EPollModifyError as Debug>::fmt(self, f)
	}
}

impl error::Error for EPollModifyError
{
}
