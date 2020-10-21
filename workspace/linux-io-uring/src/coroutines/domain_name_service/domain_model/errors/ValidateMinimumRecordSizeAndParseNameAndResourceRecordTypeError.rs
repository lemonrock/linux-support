// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Miscellaneous errors.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError
{
	/// A resource record is shorter than the minimum size.
	ResourceRecordIsShorterThanMinimumSize,
	
	/// Resource record name did not parse.
	ParsedNameParser(ParsedNameParserError),
	
	/// A resource record is shorter than the minimum size (after parsing the Name).
	ResourceRecordIsShorterThanMinimumSizeAfterParsingName,
}

impl Display for ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError::*;
		
		match self
		{
			&ParsedNameParser(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<ParsedNameParserError> for ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError
{
	#[inline(always)]
	fn from(value: ParsedNameParserError) -> Self
	{
		ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError::ParsedNameParser(value)
	}
}
