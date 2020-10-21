// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `NSEC` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NSECHandleRecordTypeError
{
	/// Resource data for resource record type `NSEC` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Error parsing a next domain name.
	NextDomainName(ParsedNameParserError),
	
	/// Error parsing type bitmaps.
	TypeBitmapsParse(TypeBitmapsParseError),
}

impl Display for NSECHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NSECHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NSECHandleRecordTypeError::*;
		
		match self
		{
			&NextDomainName(ref error) => Some(error),
			
			&TypeBitmapsParse(ref error) => Some(error),
			
			_ => None,
		}
	}
}
