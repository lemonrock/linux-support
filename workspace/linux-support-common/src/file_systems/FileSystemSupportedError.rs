// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A File System Supported error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileSystemSupportedError
{
	/// Linux kernel does not support file system.
	Unsupported(FileSystemType),

	/// File system has associated devices (ie is not 'nodev').
	HasAssociatedDevices(FileSystemType),
}

impl Display for FileSystemSupportedError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FileSystemSupportedError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::FileSystemSupportedError::*;

		match self
		{
			&Unsupported(..) => None,
			&HasAssociatedDevices(..) => None,
		}
	}
}
