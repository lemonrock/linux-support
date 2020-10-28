// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum Answer<'label, N: Name<'label>>
{
	Answered,
	
	NoDomain
	{
		response_type: NoDomainResponseType<'label, N>,
		
		/// This is a direct child of `response_type.authority_name`.
		most_canonical_name: N,
	},

	NoData
	{
		response_type: NoDataResponseType<'label, N>,
		
		/// This is a direct child of `response_type.authority_name`.
		most_canonical_name: N,
	},
	
	Referral
	{
		authority_name: N,
		
		/// These are for `authority_name`.
		name_servers: Present<N>,
		
		/// This is a direct child of `authority_name`.
		most_canonical_name: N,
	},
}

impl<'label, N: Name<'label>> Answer<'label, N>
{
	#[inline(always)]
	fn is_referral(&self) -> bool
	{
		use self::Answer::*;
		
		match self
		{
			Answered => false,
			
			NoDomain { response_type, .. } => response_type.is_implicit_referral(),
			
			NoData { response_type, .. } => response_type.is_implicit_referral(),
			
			Referral { .. } => true,
		}
	}
}
