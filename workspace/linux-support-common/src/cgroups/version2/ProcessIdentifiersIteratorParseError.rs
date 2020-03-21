/// Parse error of a process identifier.
#[derive(Debug)]
pub enum ProcessIdentifiersIteratorParseError
{
	/// Input error.
	Input(io::Error),

	/// Could not parse process identifier.
	CouldNotParseProcessIdentifier(ParseIntError),

	/// Process identifier can not be zero.
	ProcessIdentifierCanNotBeZero,
}

impl Display for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ProcessIdentifiersIteratorParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&CouldNotParseProcessIdentifier(ref source) => Some(source),

			&ProcessIdentifierCanNotBeZero => None,
		}
	}
}

impl From<io::Error> for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ProcessIdentifiersIteratorParseError::Input(value)
	}
}

impl From<ParseIntError> for ProcessIdentifiersIteratorParseError
{
	#[inline(always)]
	fn from(value: ParseIntError) -> Self
	{
		ProcessIdentifiersIteratorParseError::CouldNotParseProcessIdentifier(value)
	}
}
