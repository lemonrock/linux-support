// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global System V memory segment configuration error kind.
#[derive(Debug)]
pub enum GlobalSystemVMemorySegmentConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeMaximumMessageSize(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfQueueIdentifiers(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumQueueSizeInBytes(io::Error),
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
			&CouldNotChangeMaximumMessageSize(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfQueueIdentifiers(ref cause) => Some(cause),

			&CouldNotChangeMaximumQueueSizeInBytes(ref cause) => Some(cause),
		}
	}
}
