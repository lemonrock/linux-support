// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `NAPTR` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NAPTRHandleRecordTypeError
{
	/// Resource data for resource record type `NAPTR` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Character string was invalid.
	CharacterStrings(NoCharacterStringsError),
	
	/// Resource data for resource record type `NAPTR` is missing the flags field.
	IsMissingFlags,
	
	/// Flags character string was invalid.
	FlagsCharacterString(ParsedCharacterStringLengthIncorrectError),
	
	/// Resource data for resource record type `NAPTR` is missing the services field.
	IsMissingServices,
	
	/// Services character string was invalid.
	ServicesCharacterString(ParsedCharacterStringLengthIncorrectError),
	
	/// Resource data for resource record type `NAPTR` is missing the regular expression field.
	IsMissingRegularExpression,
	
	/// Regular expression character string was invalid.
	RegularExpressionCharacterString(ParsedCharacterStringLengthIncorrectError),
	
	/// Resource data for resource record type `NAPTR` has data left over.
	HasDataLeftOver,
	
	/// Error domain name.
	DomainName(ParsedNameParserError),
	
	/// Resource data for resource record type `NAPTR` has both a regular expression and a domain name.
	HasBothARegularExpressionAndADomainName,

	/// Flag byte was not alphanumeric
	FlagByteNotAlphanumeric(u8),
}

impl Display for NAPTRHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NAPTRHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NAPTRHandleRecordTypeError::*;
		
		match self
		{
			&CharacterStrings(ref error) => Some(error),
			
			&DomainName(ref error) => Some(error),
			
			&FlagsCharacterString(ref error) => Some(error),
			
			&ServicesCharacterString(ref error) => Some(error),
			
			&RegularExpressionCharacterString(ref error) => Some(error),
			
			_ => None,
		}
	}
}
