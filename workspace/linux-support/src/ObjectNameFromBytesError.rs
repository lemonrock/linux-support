// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectNameFromBytesError
{
	/// More than `MaximumLengthExcludingAsciiNul` bytes.
	TooLong(usize),

	/// When passing bytes that are not ASCII NUL terminated, an ASCII NUL was encountered
	ContainsAsciiNullWhereItShouldNot,

	/// When passing bytes that are ASCII NUL terminated, an ASCII NUL was expected.
	DoesNotEndWithAsciiNull,

	/// When passing bytes that are ASCII NUL terminated, the byte length including an ASCII NUL was empty.
	Empty,
}

impl Display for ObjectNameFromBytesError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ObjectNameFromBytesError
{
}
