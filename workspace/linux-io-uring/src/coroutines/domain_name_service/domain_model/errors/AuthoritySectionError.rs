// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Authority section error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AuthoritySectionError<E: error::Error>
{
	/// Miscellaneous errors.
	ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError),
	
	/// Handle record type error.
	HandleRecordType(HandleRecordTypeError<E>),
	
	/// Too many resource records in the authority section for the size of the message.
	ResourceRecordsOverflowSection,
	
	/// The error code (`RCODE`) was `NXDOMAIN` but the answer section contained one or more answers (excluding `CNAME` and `DNAME` resource records).
	ResponseHadNoSuchDomainErrorCodeButContainsAnAnswer,
	
	/// RFC 2308, Section 3, Paragraph 1.
	AuthoritativeServersMustReturnAStartOfAuthorityRecord,
}

impl<E: error::Error> Display for AuthoritySectionError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error> error::Error for AuthoritySectionError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::AuthoritySectionError::*;
		
		match self
		{
			&ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ref error) => Some(error),
			
			&HandleRecordType(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError> for AuthoritySectionError<E>
{
	#[inline(always)]
	fn from(value: ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError) -> Self
	{
		AuthorityError::ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(value)
	}
}

impl<E: error::Error> From<ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError> for AuthoritySectionError<E>
{
	#[inline(always)]
	fn from(value: HandleRecordType) -> Self
	{
		AuthoritySectionError::HandleRecordType(value)
	}
}
