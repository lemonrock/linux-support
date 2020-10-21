// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Authoritative and Authenticated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuthoritativeAndAuthenticated
{
	IsNeitherAuthoritativeOrAutenticated,
	
	IsAuthoritativeAnswer,

	IsAuthenticatedData,
}

impl AuthoritativeAndAuthenticated
{
	#[inline(always)]
	pub(crate) fn parse(is_authoritative_answer: bool, is_authenticated_data: bool) -> Result<Self, MessageHeaderError>
	{
		use self::AuthoritativeAndAuthenticated::*;
		
		match (is_authenticated, is_authoritative)
		{
			(true, true) => Err(ResponseWasAuthoritativeButHasTheAuthenticatedDataBitSet),
			
			(true, false) => Ok(IsAuthoritativeAnswer),
			
			(false, true) => Ok(IsAuthenticatedData),
			
			(false, false) => Ok(IsNeitherAuthoritativeOrAutenticated),
		}
	}
	
	#[inline(always)]
	pub(crate) fn is_authoritative_answer(self) -> bool
	{
		matches!(self, AuthoritativeAndAuthenticated::IsAuthoritativeAnswer)
	}
	
	#[inline(always)]
	pub(crate) fn is_authenticated_data(self) -> bool
	{
		matches!(self, AuthoritativeAndAuthenticated::IsAuthenticatedData)
	}
}
