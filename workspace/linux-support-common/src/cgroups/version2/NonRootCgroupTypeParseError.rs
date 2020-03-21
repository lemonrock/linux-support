/// A parse error.
#[derive(Debug)]
pub enum NonRootCgroupTypeParseError
{
	/// Input error.
	Input(io::Error),

	/// Does not end with line feed.
	DoesNotEndWithLineFeed,

	/// Invalid type name.
	InvalidTypeName
	{
		/// Name.
		name: String,
	},
}

impl Display for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)>
	{
		use self::NonRootCgroupTypeParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&DoesNotEndWithLineFeed => None,

			&InvalidTypeName { .. } => None,
		}
	}
}

impl From<io::Error> for NonRootCgroupTypeParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		NonRootCgroupTypeParseError::Input(value)
	}
}
