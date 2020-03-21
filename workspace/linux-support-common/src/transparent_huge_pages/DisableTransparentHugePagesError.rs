/// Disable Transparent Huge Pages (THP) error.
#[derive(Debug)]
pub enum DisableTransparentHugePagesError
{
	/// Could not disable defragmentation.
	Defragmentation(io::Error),

	/// Could not disable usage.
	Usage(io::Error),
}

impl Display for DisableTransparentHugePagesError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DisableTransparentHugePagesError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DisableTransparentHugePagesError::*;

		match self
		{
			&Defragmentation(ref error) => Some(error),

			&Usage(ref error) => Some(error),
		}
	}
}
