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
