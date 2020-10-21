// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `DelegationSigner` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DelegationSignerHandleRecordTypeError
{
	/// Resource data for resource record type `DS` or `CDS` has an incorrect length (value in tuple).
	HasAnIncorrectLength(DataType, usize),
	
	/// Reserved.
	DigestAlgorithmTypeIsReservedByRfc3658(DataType),
	
	/// A `DS` or `CDS` resource record has digest data that has an incorrect length for the digest type.
	HasADigestLengthThatIsIncorrectForTheDigestType(DataType, usize),
	
	/// Security algorithm error.
	SecurityAlgorithm(SecurityAlgorithmHandleRecordTypeError),
}

impl Display for DelegationSignerHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DelegationSignerHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::RRSIGHandleRecordTypeError::*;
		
		match self
		{
			&SecurityAlgorithm(ref error) => Some(error),
			
			_ => None,
		}
	}
}
