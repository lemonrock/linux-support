// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Resource type in wrong section.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ResourceTypeInWrongSectionError
{
	/// A record type was present in the answer section which should not have been (eg it was not queried for and is not `CNAME` or `DNAME`).
	ResourceRecordTypeIsNotValidInAnswerSectionIfNotRequestedByQuery(DataType),
	
	/// A record type was present in the authority section which should not have been (only `SOA` records are allowed).
	ResourceRecordTypeIsNotValidInAuthoritySection(DataType),
	
	/// A `SOA` record type was present a section it should not have been in.
	StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection,
	
	/// An `OPT` record type was present a section it should not have been in.
	///
	/// `OPT` records are only permitted in the additional section.
	ExtendedDnsOptResourceRecordTypeIsNotPermittedOutsideOfAnAdditionalSection,
}

impl Display for ResourceTypeInWrongSectionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ResourceTypeInWrongSectionError
{
}
