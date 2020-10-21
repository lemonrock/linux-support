// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `KX` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum KXHandleRecordTypeError
{
	/// Resource data for resource record type `KX` has too short a length (value in tuple).
	HasTooShortALength(usize),
	
	/// Resource data for resource record type `KX` has data remaining after the key exchange server name.
	DataRemainingAfterKeyExchangeServerName,
	
	/// Error parsing a name.
	KeyExchangeServerName(ParsedNameParserError),
}

impl Display for KXHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for KXHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::KXHandleRecordTypeError::*;
		
		match self
		{
			&KeyExchangeServerName(ref error) => Some(error),
			
			_ => None,
		}
	}
}
