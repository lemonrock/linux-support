/// Namespace inode parse error.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum NamespaceInodeParseError
{
	Input(io::Error),

	NamespaceLinkIsNotUtf8,

	NamespaceLinkIsTooShort,

	NamespaceLinkDoesNotStartWithNamespace,

	NamespaceLinkDoesNotContainColonSquareBracket,

	NamespaceLinkDoesNotEndWithSquareBracket,

	CouldNotParseInodeValue(ParseIntError),
}

impl Display for NamespaceInodeParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NamespaceInodeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NamespaceInodeParseError::*;

		match self
		{
			&Input(ref source) => Some(source),

			&NamespaceLinkIsNotUtf8 => None,

			&NamespaceLinkIsTooShort => None,

			&NamespaceLinkDoesNotStartWithNamespace => None,

			&NamespaceLinkDoesNotContainColonSquareBracket => None,

			&NamespaceLinkDoesNotEndWithSquareBracket => None,

			&CouldNotParseInodeValue(ref source) => Some(source),
		}
	}
}

impl From<io::Error> for NamespaceInodeParseError
{
	#[inline(always)]
	fn from(value: io::Error) -> Self
	{
		NamespaceInodeParseError::Input(value)
	}
}

impl From<ParseIntError> for NamespaceInodeParseError
{
	#[inline(always)]
	fn from(value: ParseIntError) -> Self
	{
		NamespaceInodeParseError::CouldNotParseInodeValue(value)
	}
}
