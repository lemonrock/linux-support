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

	#[allow(missing_docs)]
	GlobalSystemVMessageQueueConfiguration(GlobalSystemVMessageQueueConfigurationError),

	#[allow(missing_docs)]
	GlobalInotifyConfiguration(GlobalInotifyConfigurationError),

	#[allow(missing_docs)]
	GlobalEPollConfiguration(GlobalEPollConfigurationError),

	#[allow(missing_docs)]
	GlobalLinuxKernelSamePageMergingConfiguration(GlobalLinuxKernelSamePageMergingConfigurationError),

	#[allow(missing_docs)]
	GlobalLinuxKernelAsynchronousIoConfiguration(GlobalLinuxKernelAsynchronousIoConfigurationError),

	#[allow(missing_docs)]
	GlobalFileHandleConfiguration(GlobalFileHandleConfigurationError),

	#[allow(missing_docs)]
	GlobalFileDescriptorConfiguration(GlobalFileDescriptorConfigurationError),

	#[allow(missing_docs)]
	GlobalLinuxModuleConfiguration(GlobalLinuxModuleConfigurationError),

	#[allow(missing_docs)]
	GlobalKernelPanicConfiguration(GlobalKernelPanicConfigurationError),

	#[allow(missing_docs)]
	GlobalSecurityConfiguration(GlobalSecurityConfigurationError),

	#[allow(missing_docs)]
	GlobalTransparentHugePagesConfiguration(GlobalTransparentHugePagesConfigurationError),

	#[allow(missing_docs)]
	GlobalLinuxKernelCommandLineConfiguration(GlobalLinuxKernelCommandLineConfigurationError),

	#[allow(missing_docs)]
	GlobalSwapConfiguration(GlobalSwapConfigurationError),

	#[allow(missing_docs)]
	GlobalNetworkConfiguration(GlobalNetworkConfigurationError),

	#[allow(missing_docs)]
	GlobalBpfConfiguration(GlobalBpfConfigurationError),
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
			&GlobalSchedulingConfiguration(ref cause) => Some(cause),

			&GlobalPipeConfiguration(ref cause) => Some(cause),

			&GlobalFileLeasingConfiguration(ref cause) => Some(cause),

			&GlobalPosixMessageQueueConfiguration(ref cause) => Some(cause),

			&GlobalSystemVMessageQueueConfiguration(ref cause) => Some(cause),

			&GlobalInotifyConfiguration(ref cause) => Some(cause),

			&GlobalEPollConfiguration(ref cause) => Some(cause),

			&GlobalLinuxKernelSamePageMergingConfiguration(ref cause) => Some(cause),

			&GlobalLinuxKernelAsynchronousIoConfiguration(ref cause) => Some(cause),

			&GlobalFileHandleConfiguration(ref cause) => Some(cause),

			&GlobalFileDescriptorConfiguration(ref cause) => Some(cause),

			&GlobalLinuxModuleConfiguration(ref cause) => Some(cause),

			&GlobalKernelPanicConfiguration(ref cause) => Some(cause),

			&GlobalSecurityConfiguration(ref cause) => Some(cause),

			&GlobalTransparentHugePagesConfiguration(ref cause) => Some(cause),

			&GlobalLinuxKernelCommandLineConfiguration(ref cause) => Some(cause),

			&GlobalSwapConfiguration(ref cause) => Some(cause),

			&GlobalNetworkConfiguration(ref cause) => Some(cause),

			&GlobalBpfConfiguration(ref cause) => Some(cause),
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

impl From<GlobalSystemVMessageQueueConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSystemVMessageQueueConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSystemVMessageQueueConfiguration(cause)
	}
}

impl From<GlobalInotifyConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalInotifyConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalInotifyConfiguration(cause)
	}
}

impl From<GlobalEPollConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalEPollConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalEPollConfiguration(cause)
	}
}

impl From<GlobalLinuxKernelSamePageMergingConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxKernelSamePageMergingConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalLinuxKernelSamePageMergingConfiguration(cause)
	}
}

impl From<GlobalLinuxKernelAsynchronousIoConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxKernelAsynchronousIoConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalLinuxKernelAsynchronousIoConfiguration(cause)
	}
}

impl From<GlobalFileHandleConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileHandleConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileHandleConfiguration(cause)
	}
}

impl From<GlobalFileDescriptorConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalFileDescriptorConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalFileDescriptorConfiguration(cause)
	}
}

impl From<GlobalLinuxModuleConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxModuleConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalLinuxModuleConfiguration(cause)
	}
}

impl From<GlobalKernelPanicConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalKernelPanicConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalKernelPanicConfiguration(cause)
	}
}

impl From<GlobalSecurityConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSecurityConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSecurityConfiguration(cause)
	}
}

impl From<GlobalTransparentHugePagesConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalTransparentHugePagesConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalTransparentHugePagesConfiguration(cause)
	}
}

impl From<GlobalLinuxKernelCommandLineConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxKernelCommandLineConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalLinuxKernelCommandLineConfiguration(cause)
	}
}

impl From<GlobalSwapConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSwapConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalSwapConfiguration(cause)
	}
}

impl From<GlobalNetworkConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalNetworkConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalNetworkConfiguration(cause)
	}
}

impl From<GlobalBpfConfigurationError> for GlobalConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalBpfConfigurationError) -> Self
	{
		GlobalConfigurationError::GlobalBpfConfiguration(cause)
	}
}
