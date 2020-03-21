// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Master process error.
#[derive(Debug)]
pub enum MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	/// Reading file systems.
	ReadingFileSystems(io::Error),

	/// Unsupported file system.
	UnsupportedFileSystem(FileSystemSupportedError),
}

impl Display for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MasterProcessAdditionalLinuxKernelCommandLineValidationsError::*;

		match self
		{
			&ReadingFileSystems(ref error) => Some(error),

			&UnsupportedFileSystem(ref error) => Some(error),
		}
	}
}

impl From<FileSystemSupportedError> for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn from(error: FileSystemSupportedError) -> Self
	{
		MasterProcessAdditionalLinuxKernelCommandLineValidationsError::UnsupportedFileSystem(error)
	}
}
