// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global System V memory segment configuration error kind.
#[derive(Debug)]
pub enum GlobalSystemVMemorySegmentConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeMaximumMemorySegmentSize(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfMemorySegmentIdentifiers(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfSharedMemoryPages(io::Error),
}

impl Display for GlobalSystemVMemorySegmentConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalSystemVMemorySegmentConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalSystemVMemorySegmentConfigurationError::*;

		match self
		{
			&CouldNotChangeMaximumMemorySegmentSize(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfMemorySegmentIdentifiers(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfSharedMemoryPages(ref cause) => Some(cause),
		}
	}
}
