// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Authority error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AuthorityError
{
	/// A nameserver (`NS`) record in the authority section is not for the final name in a canonical name chain.
	NameServerRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain,
	
	/// A start-of-authority (`SOA`) record in the authority section is not for the final name in a canonical name chain.
	StartOfAuthorityRecordInAuthoritySectionIsNotForFinalNameInCanonicalNameChain,

	/// More than one start-of-authority (`SOA`) record in the authority section.
	MoreThanOneStartOfAuthorityRecordInAuthoritySection,
}

impl Display for AuthorityError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for AuthorityError
{
}
