// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Query section error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum QuerySectionError
{
	/// Query name.
	QueryName(ParsedNameParserError),
	
	/// DNS `QCLASS` is reserved (including for private use), unassigned or obsolete (ie Chaos or Hesiod).
	///
	/// Tuple contains value.
	///
	/// See [IANA](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-2) and RFC 6895 for further details.
	ClassIsReservedUnassignedOrObsolete(BigEndianU16),
	
	/// Response `QTYPE` did not match.
	ResponseWasForADifferentDataType,
	
	/// Response `QNAME` did not match.
	ResponseWasForADifferentName,
}

impl Display for QuerySectionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for QuerySectionError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::QuerySectionError::*;
		
		match self
		{
			&QueryName(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<ParsedNameParserError> for QuerySectionError
{
	#[inline(always)]
	fn from(value: ParsedNameParserError) -> Self
	{
		QuerySectionError::QueryName(value)
	}
}
