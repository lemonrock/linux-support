// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Security algorithm error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SecurityAlgorithmHandleRecordTypeError
{
	/// The security algorithm DS-Delete should not be used for this resource record.
	ShouldNotBeUsedForThisResourceRecordType(DataType, u8),
	
	/// The security alogrithm type is reserved (number in tuple).
	TypeIsReservedByRfc6725(DataType, u8),
	
	/// A reserved security algorithm type (number in tuple).
	TypeIsReservedByRfc6014(DataType, u8),
	
	/// Reserved.
	TypeIsReservedByRfc4034(DataType),
}

impl Display for SecurityAlgorithmHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SecurityAlgorithmHandleRecordTypeError
{
}
