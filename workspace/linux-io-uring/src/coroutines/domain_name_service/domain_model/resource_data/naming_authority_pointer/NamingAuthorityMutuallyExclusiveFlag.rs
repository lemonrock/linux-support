// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A mutually exclusive flag.
///
/// See RFC 2915, Section NAPTR RR Format, Flags, paragraph 2.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NamingAuthorityMutuallyExclusiveFlag
{
	/// The next look up should be for `SRV` records.
	S,

	/// The next look up should be for `A` or `AAAA` records (or for obsolete `A6` records).
	A,

	/// The output of the Regexp field is an URI that adheres to the 'absoluteURI' production found in the ABNF of RFC 2396.
	U,
	
	/// The remainder of the application side algorithm shall be carried out in a Protocol-specific fashion.
	///
	/// The new set of rules is identified by the Protocol specified in the Services field
	P,
}

impl NamingAuthorityMutuallyExclusiveFlag
{
	/// Is this a terminal flag that halts the looping rewrite algorithm?
	#[inline(always)]
	pub fn is_terminal(self) -> bool
	{
		use self::NamingAuthorityMutuallyExclusiveFlag::*;
		
		match self
		{
			S | A | U => true,
			
			P => false,
		}
	}
}
