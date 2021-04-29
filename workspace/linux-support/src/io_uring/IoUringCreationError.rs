// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug)]
pub enum IoUringCreationError
{
	#[allow(missing_docs)]
	CouldNotMemoryMap(CreationError),

	#[allow(missing_docs)]
	CouldNotAdviseDontFork(MemoryAdviceError),

	#[allow(missing_docs)]
	CouldNotCreateIoUringFileDescriptor(io::Error),
}

impl From<CreationError> for IoUringCreationError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		IoUringCreationError::CouldNotMemoryMap(error)
	}
}

impl Display for IoUringCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for IoUringCreationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::IoUringCreationError::*;

		match self
		{
			&CouldNotMemoryMap(ref cause) => Some(cause),

			&CouldNotAdviseDontFork(ref cause) => Some(cause),

			&CouldNotCreateIoUringFileDescriptor(ref cause) => Some(cause),
		}
	}
}
