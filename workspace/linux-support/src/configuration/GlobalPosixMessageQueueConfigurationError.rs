// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global POSIX message queue configuration error kind.
#[derive(Debug)]
pub enum GlobalPosixMessageQueueConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeDefaultMaximumNumberOfMessagesInAQueue(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumMaximumNumberOfMessagesInAQueue(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeDefaultMaximumMessageSize(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumMaximumMessageSize(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfQueues(io::Error),
}

impl Display for GlobalPosixMessageQueueConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalPosixMessageQueueConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalPosixMessageQueueConfigurationError::*;

		match self
		{
			&CouldNotChangeDefaultMaximumNumberOfMessagesInAQueue(ref cause) => Some(cause),

			&CouldNotChangeMaximumMaximumNumberOfMessagesInAQueue(ref cause) => Some(cause),

			&CouldNotChangeDefaultMaximumMessageSize(ref cause) => Some(cause),

			&CouldNotChangeMaximumMaximumMessageSize(ref cause) => Some(cause),

			&CouldNotChangeMaximumNumberOfQueues(ref cause) => Some(cause),
		}
	}
}
