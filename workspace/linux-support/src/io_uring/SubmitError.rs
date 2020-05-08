// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error submitting I/O requests.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubmitError
{
	/// The kernel was unable to allocate memory for the request, or otherwise ran out of resources to handle it.
	///
	/// The application should wait for some completions and try again.
	TryAgain,

	/// The application is attempting to overcommit the number of requests it can have pending.
	///
	/// The application should wait for some completions and try again.
	///
	/// May occur if the application tries to queue more requests than there is room for in the Completion Queue (CQ) ring.
	Busy,
}

impl Display for SubmitError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SubmitError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::SubmitError::*;

		match self
		{
			&TryAgain => None,

			&Busy => None,
		}
	}
}
