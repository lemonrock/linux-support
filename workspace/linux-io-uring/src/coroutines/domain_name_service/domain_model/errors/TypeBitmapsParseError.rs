// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// TypeBitmaps parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TypeBitmapsParseError
{
	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect length (value in tuple).
	HasAnOverflowingBlockLength(DataType, usize),
	
	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has a repeated or decreasing window number.
	HasARepeatedOrDecreasingWindowNumber(DataType),
	
	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has a zero bitmap length (value in tuple).
	HasAZeroBitmapLength(DataType),
	
	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect bitmap length (value in tuple).
	HasAnIncorrectBitmapLength(DataType, usize),
	
	/// Resource data for resource record type `CSYNC`, `NSEC` or `NSEC3` has an incorrect bitmap length (value in tuple).
	HasAnOverflowingBitmapLength(DataType, usize),
}

impl Display for TypeBitmapsParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TypeBitmapsParseError
{
}
