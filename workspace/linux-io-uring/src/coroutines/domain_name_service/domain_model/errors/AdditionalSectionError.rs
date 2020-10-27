// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Additional section error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AdditionalSectionError<E: error::Error>
{
	/// Miscellaneous errors.
	ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError),
	
	/// Handle record type error.
	HandleRecordType(HandleRecordTypeError<E>),

	/// Too many resource records in the additional section for the size of the message.
	ResourceRecordsOverflowSection,
	
	/// Response did not contain an Extended DNS `OPT` meta resource record.
	ResponseDidNotContainAnExtendedDnsOptMetaResourceRecord,
	
	/// Extended DNS 'OPT' record error.
	OPT(ExtendedDnsError),
}

impl<E: error::Error> Display for AdditionalSectionError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error> error::Error for AdditionalSectionError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::AdditionalSectionError::*;
		
		match self
		{
			&ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ref error) => Some(error),
			
			&HandleRecordType(ref error) => Some(error),
			
			&OPT(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError> for AdditionalSectionError<E>
{
	#[inline(always)]
	fn from(value: ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError) -> Self
	{
		AdditionalSectionError::ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(value)
	}
}

impl<E: error::Error> From<ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError> for AdditionalSectionError<E>
{
	#[inline(always)]
	fn from(value: HandleRecordTypeError<E>) -> Self
	{
		AdditionalSectionError::HandleRecordType(value)
	}
}

impl<E: error::Error> From<ExtendedDnsError> for AdditionalSectionError<E>
{
	#[inline(always)]
	fn from(value: ExtendedDnsError) -> Self
	{
		AdditionalSectionError::OPT(value)
	}
}
