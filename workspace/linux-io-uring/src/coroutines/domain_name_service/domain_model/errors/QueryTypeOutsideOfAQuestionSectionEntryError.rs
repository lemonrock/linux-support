// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Query type outside of a question section entry.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum QueryTypeOutsideOfAQuestionSectionEntryError
{
	/// Query type (`QTYPE`) `IXFR` is in a resource record.
	IXFR,
	
	/// Query type (`QTYPE`) `AXFR` is in a resource record.
	AXFR,
	
	/// Query type (`QTYPE`) `MAILB` is in a resource record.
	MAILB,
	
	/// Query type (`QTYPE`) `MAILA` is in a resource record.
	MAILA,
	
	/// Query type (`QTYPE`) `*` is in a resource record.
	Asterisk,
}

impl Display for QueryTypeOutsideOfAQuestionSectionEntryError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for QueryTypeOutsideOfAQuestionSectionEntryError
{
}
