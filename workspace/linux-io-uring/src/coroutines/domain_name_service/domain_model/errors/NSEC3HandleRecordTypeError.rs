// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `NSEC3` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NSEC3HandleRecordTypeError
{
	/// Resource data for resource record type `NSEC3` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Resource data for resource record type `NSEC3` has an overflowing salt length.
	HasAnOverflowingSaltLength(usize),
	
	/// Resource data for resource record type `NSEC3` has a reserved hash algorithm.
	HasAReservedHashAlgorithm,
	
	/// Resource data for resource record type `NSEC3` has an incorrect hash length for a SHA-1 hash.
	HasAnIncorrectHashLengthForASha1Hash(usize),
	
	/// Resource data for resource record type `NSEC3` has an overflowing hash length.
	HasAnOverflowingHashLength(usize),
	
	/// Error parsing type bitmaps.
	TypeBitmapsParse(TypeBitmapsParseError),
}

impl Display for NSEC3HandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NSEC3HandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::NSEC3HandleRecordTypeError::*;
		
		match self
		{
			&TypeBitmapsParse(ref error) => Some(error),
			
			_ => None,
		}
	}
}
