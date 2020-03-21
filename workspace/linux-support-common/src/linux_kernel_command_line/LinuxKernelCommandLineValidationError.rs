// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux kernel command line validation error.
#[derive(Debug)]
pub enum LinuxKernelCommandLineValidationError<AdditionalLinuxKernelCommandLineValidationFailedError: error::Error>
{
	/// CPUs are invalid.
	CpusInvalid(String),

	/// Huge Page Sizes are invalid.
	HugePageSizesInvalid(String),

	/// Incompatible validations.
	IncompatibleValidations(String),

	/// Additional, process-specific checks failed.
	AdditionalLinuxKernelCommandLineValidationFailed(AdditionalLinuxKernelCommandLineValidationFailedError)
}

impl<AdditionalLinuxKernelCommandLineValidationFailedError: error::Error> Display for LinuxKernelCommandLineValidationError<AdditionalLinuxKernelCommandLineValidationFailedError>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<AdditionalLinuxKernelCommandLineValidationFailedError: 'static + error::Error> error::Error for LinuxKernelCommandLineValidationError<AdditionalLinuxKernelCommandLineValidationFailedError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::LinuxKernelCommandLineValidationError::*;

		match self
		{
			&CpusInvalid(..) => None,

			&HugePageSizesInvalid(..) => None,

			&IncompatibleValidations(..) => None,

			&AdditionalLinuxKernelCommandLineValidationFailed(ref source) => Some(source),
		}
	}
}
