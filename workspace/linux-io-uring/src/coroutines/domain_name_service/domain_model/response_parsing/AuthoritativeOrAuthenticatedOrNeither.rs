// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Authoritative or Authenticated or Both.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AuthoritativeOrAuthenticatedOrNeither
{
	/// Only if the name server the query was sent to was the authoritative one for this domain.
	///
	/// Some name servers may return a `REFUSED` code instead and not set this bit.
	IsAuthoritativeAnswer,

	/// Only if the domain requested has DNSSec data, eg `strommq.com.` (sic).
	IsAuthenticatedData,
	
	/// If the domain does not have DNSSec data (ie signatures, etc).
	IsNeitherAuthoritativeOrAuthenticated,
}

impl AuthoritativeOrAuthenticatedOrNeither
{
	#[inline(always)]
	pub(crate) fn parse(is_authoritative_answer: bool, is_authenticated_data: bool) -> Result<Self, MessageHeaderError>
	{
		use self::AuthoritativeOrAuthenticatedOrNeither::*;
		
		match (is_authoritative_answer, is_authenticated_data)
		{
			(true, true) => Err(MessageHeaderError::ResponseWasAuthoritativeButHasTheAuthenticatedDataBitSet),
			
			(true, false) => Ok(IsAuthoritativeAnswer),
			
			(false, true) => Ok(IsAuthenticatedData),
			
			(false, false) => Ok(IsNeitherAuthoritativeOrAuthenticated),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_authoritative_answer(self) -> bool
	{
		matches!(self, AuthoritativeOrAuthenticatedOrNeither::IsAuthoritativeAnswer)
	}
	
	/// This is the `AD` bit.
	///
	/// It was defined in RFC 2535.
	/// It was altered by RFC 3655.
	/// It was then redefined as part of DNSSec in RFCs 4033, 4034 and 4035, particularly, RFC 4035, Section 3.2.3 The AD Bit.
	/// It was then further clarified in RFC 6840!
	#[inline(always)]
	pub(crate) fn is_authenticated_data(self) -> bool
	{
		matches!(self, AuthoritativeOrAuthenticatedOrNeither::IsAuthenticatedData)
	}
}
