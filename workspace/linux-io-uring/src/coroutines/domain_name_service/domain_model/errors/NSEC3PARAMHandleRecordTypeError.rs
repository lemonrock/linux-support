// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `NSEC3PARAM` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NSEC3PARAMHandleRecordTypeError
{
	/// Resource data for resource record type `NSEC3PARAM` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Resource data for resource record type `NSEC3PARAM` has an overflowing salt length.
	HasAnOverflowingSaltLength(usize),
	
	/// Resource data for resource record type `NSEC3PARAM` has a reserved hash algorithm.
	HasAReservedHashAlgorithm,
}

impl Display for NSEC3PARAMHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NSEC3PARAMHandleRecordTypeError
{
}
