// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug)]
pub enum StatParseError
{
	/// Could not open file.
	CouldNotOpenFile(io::Error),
	
	#[allow(missing_docs)]
	NoOpenBracket,
	
	#[allow(missing_docs)]
	NoCharactersBeforeOpenBracket,
	
	#[allow(missing_docs)]
	NoSpaceBeforeOpenBracket,
	
	#[allow(missing_docs)]
	NoCloseBracket,
	
	#[allow(missing_docs)]
	NoCharactersAfterCloseBracket,
	
	#[allow(missing_docs)]
	NoSpaceAfterCloseBracket,
	
	#[allow(missing_docs)]
	MissingField
	{
		/// Field name.
		name: &'static str,
	},
	
	#[allow(missing_docs)]
	NegativeValue,
	
	#[allow(missing_docs)]
	LargeValue,
	
	#[allow(missing_docs)]
	ZeroValue,
	
	/// Invalid number.
	ParseNumber(ParseNumberError),
	
	/// Could not parse command name.
	CouldNotParseCommandName(ObjectNameFromBytesError),
	
	/// Could not parse child status.
	CouldNotParseChildStatus(OutOfRangeSignalNumberError),
	
	/// Obsolete field.
	ObsoleteFieldValueWasNotZero
	{
		/// Field name.
		name: &'static str,
		
		/// Non-zero value.
		value: NonZeroU64,
	}
}

impl Display for StatParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for StatParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::StatParseError::*;

		match self
		{
			&CouldNotOpenFile(ref cause) => Some(cause),
			
			&NoOpenBracket => None,
			
			&NoCharactersBeforeOpenBracket => None,
			
			&NoSpaceBeforeOpenBracket => None,
			
			&NoCloseBracket => None,
			
			&NoCharactersAfterCloseBracket => None,
			
			&NoSpaceAfterCloseBracket => None,
			
			&NegativeValue => None,
			
			&LargeValue => None,
			
			&ZeroValue => None,
			
			&MissingField { .. } => None,
			
			&ParseNumber(ref cause) => Some(cause),

			&CouldNotParseCommandName(ref cause) => Some(cause),
			
			&ObsoleteFieldValueWasNotZero { .. } => None,
		}
	}
}

impl From<io::Error> for StatParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		StatParseError::CouldNotOpenFile(error)
	}
}

impl From<ObjectNameFromBytesError> for StatParseError
{
	#[inline(always)]
	fn from(error: ObjectNameFromBytesError) -> Self
	{
		StatParseError::CouldNotParseCommandName(error)
	}
}

impl From<OutOfRangeSignalNumberError> for StatParseError
{
	#[inline(always)]
	fn from(error: OutOfRangeSignalNumberError) -> Self
	{
		StatParseError::CouldNotParseChildStatus(error)
	}
}

impl From<ParseNumberError> for StatParseError
{
	#[inline(always)]
	fn from(error: ParseNumberError) -> Self
	{
		StatParseError::ParseNumber(error)
	}
}
