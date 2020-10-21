// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// DNS key errors.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DnsKeyHandleRecordTypeError
{
	/// Resource data for resource record type `DNSKEY` or `CDNSKEY` has an incorrect length (value in tuple).
	HasAnIncorrectLength(DataType, usize),
	
	/// Resource data for resource record type `DNSKEY` or `CDNSKEY` has an incorrect length (value in tuple) for Protocol 3.
	HasAnIncorrectLengthForProtocol3(DataType, usize),
	
	/// Security algorithm error.
	SecurityAlgorithm(SecurityAlgorithmHandleRecordTypeError),
}

impl Display for DnsKeyHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DnsKeyHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DnsKeyHandleRecordTypeError::*;
		
		match self
		{
			&SecurityAlgorithm(ref error) => Some(error),
			
			_ => None,
		}
	}
}
