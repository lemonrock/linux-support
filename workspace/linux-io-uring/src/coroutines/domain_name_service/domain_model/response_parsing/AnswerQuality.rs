// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Answer Quality.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnswerQuality
{
	Normal(AuthoritativeAndAuthenticated),
	
	DnsSecDataFailedAuthentication
	{
		is_authenticated_data: bool
	},
	
	AuthoritativeServerReportsNoDomainButThisIsNotValidated,
}

impl AnswerQuality
{
	#[inline(always)]
	pub(crate) fn has_nxdomain_error_code(self) -> bool
	{
		matches!(self, Outcome::AuthoritativeServerReportsNoDomainButThisIsNotValidated)
	}
	
	#[inline(always)]
	pub fn is_authoritative_answer(self) -> bool
	{
		use self::AnswerQuality::*;
		
		match self
		{
			Normal(authoritative_and_authenticated) => authoritative_and_authenticated.is_authoritative_answer(),
			
			DnsSecDataFailedAuthentication { .. }  => (),
			
			AuthoritativeServerReportsNoDomainButThisIsNotValidated => true,
		}
	}
	
	#[inline(always)]
	pub fn is_authenticated_data(self) -> bool
	{
		use self::AnswerQuality::*;
		
		match self
		{
			Normal(authoritative_and_authenticated) => authoritative_and_authenticated.is_authenticated_data(),
			
			DnsSecDataFailedAuthentication { is_authenticated_data }  => is_authenticated_data,
			
			AuthoritativeServerReportsNoDomainButThisIsNotValidated => false,
		}
	}
}
