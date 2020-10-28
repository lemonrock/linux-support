// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `URI` record type error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum URIHandleRecordTypeError
{
	/// Resource data for resource record type `URI` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Invalid target URI.
	InvalidTargetUri(URIError),
}

impl Display for URIHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for URIHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::URIHandleRecordTypeError::*;
		
		match self
		{
			&InvalidTargetUri(ref error) => Some(error),
			
			_ => None,
		}
	}
}
