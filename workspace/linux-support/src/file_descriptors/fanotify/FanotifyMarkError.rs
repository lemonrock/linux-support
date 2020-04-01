// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during binding of a socket instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FanotifyMarkError
{
	/// Kernel would be out of memory.
	KernelWouldBeOutOfMemory,

	/// `flags` contained `OnlyDirectory`, and `dirfd` and `path` do not specify a directory, or, the filesystem object indicated by `dirfd` and `pathname` does not exist, or, one was trying to remove a mark from an object which is not marked.
	FilePathInvalid,
}

impl Display for FanotifyMarkError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<FanotifyMarkError as Debug>::fmt(self, f)
	}
}

impl error::Error for FanotifyMarkError
{
}
