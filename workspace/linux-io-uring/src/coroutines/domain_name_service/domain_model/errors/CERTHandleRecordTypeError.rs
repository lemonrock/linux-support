// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `CERT` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CERTHandleRecordTypeError
{
	/// Resource data for resource record type `CERT` has too short a length (value in tuple).
	HasTooShortALength(usize),
	
	/// Resource data for resource record type `CERT` uses a reserved certificate type value (value in tuple).
	UsesAReservedCertificateTypeValue(u16),
	
	/// Resource data for resource record type `CERT` uses an experimental certificate type value (value in tuple).
	UsesAnExperimentalCertificateTypeValue(u16),

	/// Security algorithm error.
	SecurityAlgorithmFailed(SecurityAlgorithmHandleRecordTypeError),
}

impl Display for CERTHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CERTHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::CERTHandleRecordTypeError::*;
		
		match self
		{
			&SecurityAlgorithmFailed(ref error) => Some(error),
			
			_ => None,
		}
	}
}
