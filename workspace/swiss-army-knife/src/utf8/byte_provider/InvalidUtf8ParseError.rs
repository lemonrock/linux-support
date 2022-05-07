// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvalidUtf8ParseError<E: error::Error>
{
	#[allow(missing_docs)]
	DidNotExpectEndParsing(Utf8CharacterLength),
	
	#[allow(missing_docs)]
	OverlyLongUtf8Sequence,
	
	#[allow(missing_docs)]
	InvalidUtf8CodePoint(CharTryFromError),
	
	#[allow(missing_docs)]
	Inner(E),
}

impl<E: error::Error> From<CharTryFromError> for InvalidUtf8ParseError<E>
{
	#[inline(always)]
	fn from(cause: CharTryFromError) -> Self
	{
		InvalidUtf8ParseError::InvalidUtf8CodePoint(cause)
	}
}

impl<E: error::Error> Display for InvalidUtf8ParseError<E>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl<E: 'static + error::Error> error::Error for InvalidUtf8ParseError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use InvalidUtf8ParseError::*;
		
		match self
		{
			InvalidUtf8CodePoint(cause) => Some(cause),
			
			Inner(cause) => Some(cause),
			
			_ => None,
		}
	}
}
