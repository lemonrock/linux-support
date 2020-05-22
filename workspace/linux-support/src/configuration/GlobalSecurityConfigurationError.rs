// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global security configuration error kind.
#[derive(Debug)]
pub enum GlobalSecurityConfigurationError
{
	#[allow(missing_docs)]
	CouldNotHarden
	{
		proc_sys_kernel_file: &'static str,
		cause: io::Error,
	},
	
	#[allow(missing_docs)]
	CouldNotSetMaximumProcessIdentifiersToMaximum(io::Error),

	#[allow(missing_docs)]
	CouldNotDisableKexecLoadingUntilNextReboot(io::Error),
}

impl Display for GlobalSecurityConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalSecurityConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalSecurityConfigurationError::*;

		match self
		{
			&CouldNotHarden { ref cause, .. } => Some(cause),
			
			&CouldNotSetMaximumProcessIdentifiersToMaximum(ref cause) => Some(cause),
			
			&CouldNotDisableKexecLoadingUntilNextReboot(ref cause) => Some(cause),
		}
	}
}
