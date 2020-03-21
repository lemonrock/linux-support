/// A common process configuration execution error.
#[derive(Debug)]
pub enum DetailedProcessConfigurationError
{
	/// Process niceness adjustment failed.
	ProcessNicenessAdjustmentFailed(ProcessNicenessAdjustmentError),

	/// Process affinity setting failed.
	CouldNotSetCurrentProcessAffinity(io::Error),

	/// Could not load seccomp filters.
	CouldNotLoadSeccompFilters,
}

impl Display for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DetailedProcessConfigurationError::*;

		match self
		{
			&ProcessNicenessAdjustmentFailed(ref error) => Some(error),

			&CouldNotSetCurrentProcessAffinity(ref error) => Some(error),

			&CouldNotLoadSeccompFilters => None,
		}
	}
}

impl From<ProcessNicenessAdjustmentError> for DetailedProcessConfigurationError
{
	#[inline(always)]
	fn from(error: ProcessNicenessAdjustmentError) -> Self
	{
		DetailedProcessConfigurationError::ProcessNicenessAdjustmentFailed(error)
	}
}
