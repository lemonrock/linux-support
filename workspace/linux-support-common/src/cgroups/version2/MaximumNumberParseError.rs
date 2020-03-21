/// Errors when parsing a maximum number.
#[derive(Debug)]
pub enum MaximumNumberParseError
{
	/// Input error.
	Input(io::Error),

	/// Did not end with a line feed.
	DoesNotEndWithLineFeed,

	/// Could not parse value.
	WasNotMaximumOrAFiniteInteger(ParseIntError),
}

impl Display for MaximumNumberParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for MaximumNumberParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::MaximumNumberParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DoesNotEndWithLineFeed => None,

			&WasNotMaximumOrAFiniteInteger(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for MaximumNumberParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		MaximumNumberParseError::Input(value)
	}
}

impl From<ParseIntError> for MaximumNumberParseError
{
	#[inline(always)]
	fn from(value: ParseIntError) -> Self
	{
		MaximumNumberParseError::WasNotMaximumOrAFiniteInteger(value)
	}
}
