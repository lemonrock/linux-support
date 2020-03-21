/// Process niceness error.
#[derive(Debug)]
pub enum ProcessNicenessAdjustmentError
{
	/// Could not set current real effective user priority niceness (permission was denied in some way).
	CouldNotSetCurrentRealEffectiveUserPriorityNiceness,

	/// Could not set current process group user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessGroupPriorityNiceness,

	/// Could not set current process user priority niceness (permission was denied in some way).
	CouldNotSetCurrentProcessPriorityNiceness,

	/// Could not set current process user autogroup priority niceness.
	CouldNotSetCurrentProcessAutogroupPriorityNiceness(io::Error),
}

impl Display for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessNicenessAdjustmentError::*;

		match self
		{
			&CouldNotSetCurrentRealEffectiveUserPriorityNiceness => None,

			&CouldNotSetCurrentProcessGroupPriorityNiceness => None,

			&CouldNotSetCurrentProcessPriorityNiceness => None,

			&CouldNotSetCurrentProcessAutogroupPriorityNiceness(ref error) => Some(error),
		}
	}
}

impl From<io::Error> for ProcessNicenessAdjustmentError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ProcessNicenessAdjustmentError::CouldNotSetCurrentProcessAutogroupPriorityNiceness(error)
	}
}
