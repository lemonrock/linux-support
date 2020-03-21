/// Master process error.
#[derive(Debug)]
pub enum MasterProcessError
{
	/// Input-output error.
	InputOutput(io::Error),

	/// Could not start child process.
	CouldNotStartChildProcess(CouldNotStartChildProcessError),
}

impl Display for MasterProcessError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MasterProcessError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MasterProcessError::*;

		match self
		{
			&InputOutput(ref source) => Some(source),

			&CouldNotStartChildProcess(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for MasterProcessError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		MasterProcessError::InputOutput(value)
	}
}

impl From<CouldNotStartChildProcessError> for MasterProcessError
{
	#[inline(always)]
	fn from(value: CouldNotStartChildProcessError) -> Self
	{
		MasterProcessError::CouldNotStartChildProcess(value)
	}
}
