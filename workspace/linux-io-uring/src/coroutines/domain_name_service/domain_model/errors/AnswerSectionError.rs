// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Answer section error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnswerSectionError<E: error::Error>
{
	/// Miscellaneous errors.
	ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError),
	
	/// Handle record type error.
	HandleRecordType(HandleRecordTypeError<E>),
	
	/// More than one `CNAME` record exists in an answer section when a QueryType was `CNAME`.
	MoreThanOneCNAMERecordIsNotValidInAnswerSectionForACNAMEQuery,
	
	/// More than one `DNAME` record exists in an answer section when a QueryType was `DNAME`.
	MoreThanOneDNAMERecordIsNotValidInAnswerSectionForADNAMEQuery,
	
	/// Too many resource records in the answer section for the size of the message.
	ResourceRecordsOverflowSection,
}

impl<E: error::Error> Display for AnswerSectionError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: 'static + error::Error> error::Error for AnswerSectionError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::AnswerSectionError::*;
		
		match self
		{
			&ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(ref error) => Some(error),
			
			&HandleRecordType(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError> for AnswerSectionError<E>
{
	#[inline(always)]
	fn from(value: ValidateMinimumRecordSizeAndParseNameAndResourceRecordTypeError) -> Self
	{
		AnswerSectionError::ValidateMinimumRecordSizeAndParseNameAndResourceRecordType(value)
	}
}

impl<E: error::Error> From<HandleRecordTypeError<E>> for AnswerSectionError<E>
{
	#[inline(always)]
	fn from(value: HandleRecordTypeError<E>) -> Self
	{
		AnswerSectionError::HandleRecordType(value)
	}
}
