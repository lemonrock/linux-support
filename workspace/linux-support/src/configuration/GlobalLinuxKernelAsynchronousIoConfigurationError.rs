// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux kernel asynchronous IO (KAIO) configuration error kind.
#[derive(Debug)]
pub enum GlobalLinuxKernelAsynchronousIoConfigurationError
{
	#[allow(missing_docs)]
	CouldNotChangeMaximumNumberOfKernelAsynchronousIoEventsPerUser(io::Error),
}

impl Display for GlobalLinuxKernelAsynchronousIoConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalLinuxKernelAsynchronousIoConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalLinuxKernelAsynchronousIoConfigurationError::*;

		match self
		{
			&CouldNotChangeMaximumNumberOfKernelAsynchronousIoEventsPerUser(ref cause) => Some(error),
		}
	}
}
