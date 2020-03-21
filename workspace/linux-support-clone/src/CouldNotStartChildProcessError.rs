/// Could not start child process.
#[derive(Debug)]
pub enum CouldNotStartChildProcessError
{
	/// Input-Output error.
	InputOutput(io::Error),

	/// Could not allocate an anonymous pipe.
	CouldNotAllocateAnonymousPipe(CreationError),

	/// Could not allocate stack.
	CouldNotAllocateStack(AllocErr),

	/// Could not clone.
	CouldNotClone(CloneError),
}

impl Display for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::CouldNotStartChildProcessError::*;

		match self
		{
			&InputOutput(ref source) => Some(source),

			&CouldNotAllocateAnonymousPipe(ref source) => Some(source),

			&CouldNotAllocateStack(ref source) => Some(source),

			&CouldNotClone(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		CouldNotStartChildProcessError::InputOutput(error)
	}
}

impl From<CreationError> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		CouldNotStartChildProcessError::CouldNotAllocateAnonymousPipe(error)
	}
}

impl From<AllocErr> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: AllocErr) -> Self
	{
		CouldNotStartChildProcessError::CouldNotAllocateStack(error)
	}
}

impl From<CloneError> for CouldNotStartChildProcessError
{
	#[inline(always)]
	fn from(error: CloneError) -> Self
	{
		CouldNotStartChildProcessError::CouldNotClone(error)
	}
}
