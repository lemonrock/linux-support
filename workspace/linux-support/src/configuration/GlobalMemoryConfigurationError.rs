// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global memory configuration error kind.
#[derive(Debug)]
pub enum GlobalMemoryConfigurationError
{
	#[allow(missing_docs)]
	GlobalLinuxKernelSamePageMergingConfiguration(GlobalLinuxKernelSamePageMergingConfigurationError),

	#[allow(missing_docs)]
	GlobalTransparentHugePagesConfiguration(GlobalTransparentHugePagesConfigurationError),
	
	#[allow(missing_docs)]
	GlobalSwapConfiguration(GlobalSwapConfigurationError),

	#[allow(missing_docs)]
	GlobalOutOfMemoryConfiguration(GlobalOutOfMemoryConfigurationError),

	#[allow(missing_docs)]
	GlobalNumaBalancingConfiguration(GlobalNumaBalancingConfigurationError),

	#[allow(missing_docs)]
	GlobalNumaMemoryReclaimConfiguration(GlobalNumaMemoryReclaimConfigurationError),

	#[allow(missing_docs)]
	GlobalMemoryStatisticsConfiguration(GlobalMemoryStatisticsConfigurationError),

	#[allow(missing_docs)]
	CouldNotChangeCompactUnevictableAllowed(io::Error),

	#[allow(missing_docs)]
	CouldNotChangeSignalOnMemoryCheckFailure(io::Error),

	#[allow(missing_docs)]
	CouldNotChangePerHyperThreadPageListFraction(io::Error),
}

impl Display for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalMemoryConfigurationError::*;

		match self
		{
			&GlobalLinuxKernelSamePageMergingConfiguration(ref cause) => Some(cause),

			&GlobalTransparentHugePagesConfiguration(ref cause) => Some(cause),
			
			&GlobalSwapConfiguration(ref cause) => Some(cause),

			&GlobalOutOfMemoryConfiguration(ref cause) => Some(cause),

			&GlobalNumaBalancingConfiguration(ref cause) => Some(cause),

			&GlobalNumaMemoryReclaimConfiguration(ref cause) => Some(cause),

			&GlobalMemoryStatisticsConfiguration(ref cause) => Some(cause),

			&CouldNotChangeCompactUnevictableAllowed(ref cause) => Some(cause),

			&CouldNotChangeSignalOnMemoryCheckFailure(ref cause) => Some(cause),

			&CouldNotChangePerHyperThreadPageListFraction(ref cause) => Some(cause),
		}
	}
}

impl From<GlobalLinuxKernelSamePageMergingConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalLinuxKernelSamePageMergingConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalLinuxKernelSamePageMergingConfiguration(cause)
	}
}

impl From<GlobalTransparentHugePagesConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalTransparentHugePagesConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalTransparentHugePagesConfiguration(cause)
	}
}

impl From<GlobalSwapConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalSwapConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalSwapConfiguration(cause)
	}
}

impl From<GlobalOutOfMemoryConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalOutOfMemoryConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalOutOfMemoryConfiguration(cause)
	}
}

impl From<GlobalNumaBalancingConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalNumaBalancingConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalNumaBalancingConfiguration(cause)
	}
}

impl From<GlobalNumaMemoryReclaimConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalNumaMemoryReclaimConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalNumaMemoryReclaimConfiguration(cause)
	}
}

impl From<GlobalMemoryStatisticsConfigurationError> for GlobalMemoryConfigurationError
{
	#[inline(always)]
	fn from(cause: GlobalMemoryStatisticsConfigurationError) -> Self
	{
		GlobalMemoryConfigurationError::GlobalMemoryStatisticsConfiguration(cause)
	}
}
