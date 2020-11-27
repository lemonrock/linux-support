// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum Answer<PR: ParsedRecord>
{
	Answered
	{
		// This is the domain for which there are records (eg `AAAA`).
		//most_canonical_name: EfficientCaseFoldedName,
	
		records: OwnerNameToParsedRecordsValue<PR>,
	},
	
	NoDomain
	{
		// This is a child of `response_type.authority_name`.
		//
		// This is the domain for which there is no domain.
		//most_canonical_name: EfficientCaseFoldedName,
		
		response_type: NoDomainResponseType,
	},

	NoData
	{
		// This is a child of `response_type.authority_name`.
		//
		// This is the domain for which there is no data.
		//most_canonical_name: EfficientCaseFoldedName,
		
		response_type: NoDataResponseType,
	},
	
	Referral
	{
		// This is a child of `authority_name`.
		//most_canonical_name: EfficientCaseFoldedName,
		
		referral: AuthorityNameNameServers,
	},
}
