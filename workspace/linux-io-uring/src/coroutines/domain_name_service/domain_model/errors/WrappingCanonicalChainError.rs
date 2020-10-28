// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wrapping anonical chain error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum WrappingCanonicalChainError<E: error::Error>
{
	CanonicalChain(CanonicalChainError),

	Wrapped(E)
}

impl<E: error::Error> Display for WrappingCanonicalChainError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: 'static + error::Error> error::Error for WrappingCanonicalChainError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::WrappingCanonicalChainError::*;
		
		match self
		{
			&CanonicalChain(ref error) => Some(error),
			
			&Wrapped(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<CanonicalChainError> for WrappingCanonicalChainError<E>
{
	#[inline(always)]
	fn from(value: CanonicalChainError) -> Self
	{
		WrappingCanonicalChainError::CanonicalChain(value)
	}
}
