// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `LOC` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LOCHandleRecordTypeError
{
	/// Resource data for resource record type `LOC` has an incorrect length; for any version, it can not be empty.
	CanNotBeEmpty,
	
	/// Resource data for resource record type `LOC` has an incorrect version (value in tuple).
	InvalidVersion(u8),
	
	/// Resource data for resource record type `LOC` has an incorrect length (value in tuple).
	HasAnIncorrectLengthForVersion0(usize),
}

impl Display for LOCHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for LOCHandleRecordTypeError
{
}
