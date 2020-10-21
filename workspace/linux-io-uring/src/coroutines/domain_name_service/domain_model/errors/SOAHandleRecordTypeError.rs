// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `SOA` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SOAHandleRecordTypeError
{
	/// More than one `SOA` resource record.
	MoreThanOneStartOfAuthorityResourceRecord,
	
	/// Error parsing a `SOA` `MNAME`.
	ParseStartOfAuthorityMName(ParsedNameParserError),
	
	/// Error parsing a `SOA` `RNAME`.
	ParseStartOfAuthorityRName(ParsedNameParserError),
	
	/// Resource data for resource record type `SOA` has an invalid length after parsing `MNAME` and `RNAME`.
	StartOfAuthorityIsIncorrectSizeAfterParsingMNameAndRName,
}

impl Display for SOAHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SOAHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::SOAHandleRecordTypeError::*;
		
		match self
		{
			&ParseStartOfAuthorityMName(ref error) => Some(error),
			
			&ParseStartOfAuthorityRName(ref error) => Some(error),
			
			_ => None,
		}
	}
}
