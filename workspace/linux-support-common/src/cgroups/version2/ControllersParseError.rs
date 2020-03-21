/// Errors when parsing a maximum number.
#[derive(Debug)]
pub enum ControllersParseError
{
	/// Input error.
	Input(io::Error),

	/// Did not end with a line feed.
	DoesNotEndWithLineFeed,

	/// Invalid controller name.
	InvalidControllerName(String),

	/// Duplicate controller.
	DuplicateController(Controller),
}

impl Display for ControllersParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for ControllersParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::ControllersParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DoesNotEndWithLineFeed => None,

			&InvalidControllerName(..) => None,

			&DuplicateController(..) => None,
		}
	}
}

impl From<io::Error> for ControllersParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		ControllersParseError::Input(value)
	}
}
