// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global configuration error kind.
#[derive(Debug)]
pub enum GlobalConfigurationError
{
	#[allow(missing_docs)]
	GlobalSchedulingConfiguration(GlobalSchedulingConfigurationError),

	#[allow(missing_docs)]
	GlobalPipeConfiguration(GlobalPipeConfigurationError),

	#[allow(missing_docs)]
	GlobalFileLeasingConfiguration(GlobalFileLeasingConfigurationError),

	#[allow(missing_docs)]
	GlobalPosixMessageQueueConfiguration(GlobalPosixMessageQueueConfigurationError),
}

impl Display for GlobalConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalConfigurationError::*;

		match self
		{
			&GlobalSchedulingConfiguration(ref cause) => Some(error),

			&GlobalPipeConfiguration(ref cause) => Some(error),

			&GlobalFileLeasingConfiguration(ref cause) => Some(error),

			&GlobalPosixMessageQueueConfiguration(ref cause) => Some(error),
		}
	}
}

impl From<GlobalSchedulingConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSchedulingConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSchedulingConfiguration(cause)
	}
}

impl From<GlobalPipeConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalPipeConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalPipeConfiguration(cause)
	}
}

impl From<GlobalFileLeasingConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileLeasingConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileLeasingConfiguration(cause)
	}
}

impl From<GlobalPosixMessageQueueConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalPosixMessageQueueConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalPosixMessageQueueConfiguration(cause)
	}
}
