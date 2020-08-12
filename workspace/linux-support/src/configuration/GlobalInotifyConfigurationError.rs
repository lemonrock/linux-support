// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global inotify configuration error kind.
#[derive(Debug)]
pub enum GlobalInotifyConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfEventsThatCanBeQueued(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfInotifyInstancesPerUser(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfWatchesPerUser(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfInotifyInstancesPerUserNamespaced(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfWatchesPerUserNamespaced(io::Error),
}

impl Display for GlobalInotifyConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalInotifyConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalInotifyConfigurationError::*;

		match self
		{
			&CouldNotChangeMaximumNumberOfEventsThatCanBeQueued(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfInotifyInstancesPerUser(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfWatchesPerUser(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfInotifyInstancesPerUserNamespaced(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfWatchesPerUserNamespaced(ref cause) => Some(cause),
		}
	}
}
