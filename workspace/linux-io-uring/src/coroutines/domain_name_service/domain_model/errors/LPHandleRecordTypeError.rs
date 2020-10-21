// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `LP` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LPHandleRecordTypeError
{
	/// Resource data for resource record type `LP` has too short a length (value in tuple).
	HasTooShortALength(usize),
	
	/// Domain name.
	DomainName(ParsedNameParserError),
	
	/// Resource data for resource record type `LP` has data left over after parsing the domain name.
	HasDataLeftOver,
	
	/// Resource data for resource record type `LP` has a `domain_name` the same as the resource record's name.
	HasDomainNameSameAsRecordName,
}

impl Display for LPHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for LPHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::LPHandleRecordTypeError::*;
		
		match self
		{
			&DomainName(ref error) => Some(error),
			
			_ => None,
		}
	}
}
