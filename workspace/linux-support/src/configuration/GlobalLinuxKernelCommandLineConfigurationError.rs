// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Linux kernel command line configuration error kind.
#[derive(Debug)]
pub enum GlobalLinuxKernelCommandLineConfigurationError
{
	#[allow(missing_docs)]
	CouldNotParseLinuxKernelCommandLineParameters(io::Error),

	#[allow(missing_docs)]
	IncompatibleSettings(&'static str),

	#[allow(missing_docs)]
	InvalidHugePageSizes(&'static str),

	#[allow(missing_docs)]
	OptionalLinuxKernelCommandLineSettingChecksFailed(FailedChecks<OptionalKernelCommandLineSettingCheck>),
}

impl Display for GlobalLinuxKernelCommandLineConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for GlobalLinuxKernelCommandLineConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::GlobalLinuxKernelCommandLineConfigurationError::*;

		match self
		{
			&CouldNotParseLinuxKernelCommandLineParameters(ref cause) => Some(cause),

			&IncompatibleSettings(..) => None,

			&InvalidHugePageSizes(..) => None,

			&OptionalLinuxKernelCommandLineSettingChecksFailed(ref cause) => Some(cause),
		}
	}
}
