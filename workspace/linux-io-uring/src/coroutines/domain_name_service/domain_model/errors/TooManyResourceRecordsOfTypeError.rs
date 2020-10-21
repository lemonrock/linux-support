// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Too many resource records of type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TooManyResourceRecordsOfTypeError
{
	/// More than one `CNAME` record exists in an answer section when a QueryType was `CNAME`.
	MoreThanOneCNAMERecordIsNotValidInAnswerSectionForACNAMEQuery,
	
	/// More than one `DNAME` record exists in an answer section when a QueryType was `DNAME`.
	MoreThanOneDNAMERecordIsNotValidInAnswerSectionForADNAMEQuery,
}

impl Display for TooManyResourceRecordsOfTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for TooManyResourceRecordsOfTypeError
{
}
