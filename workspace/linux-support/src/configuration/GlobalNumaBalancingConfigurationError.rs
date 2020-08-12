// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global NUMA balancing configuration error kind.
#[derive(Debug)]
pub enum GlobalNumaBalancingConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeEnablement(io::Error),
	
	#[allow(missing_docs)]
	GlobalNumaBalancingOnConfiguration(GlobalNumaBalancingOnConfigurationError),
}

impl Display for GlobalNumaBalancingConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalNumaBalancingConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalNumaBalancingConfigurationError::*;

		match self
		{
			&CouldNotChangeEnablement(ref cause) => Some(cause),
			
			&GlobalNumaBalancingOnConfiguration(ref cause) => Some(cause),
		}
	}
}

impl From<GlobalNumaBalancingOnConfigurationError> for GlobalNumaBalancingConfigurationError
{
	fn from(value: GlobalNumaBalancingOnConfigurationError) -> Self
	{
		GlobalNumaBalancingConfigurationError::GlobalNumaBalancingOnConfiguration(value)
	}
}
