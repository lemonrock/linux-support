// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `DHCID` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DHCIDHandleRecordTypeError
{
	/// Resource data for resource record type `DHCID` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Resource data for resource record type `DHCID` has reserved identifier type code.
	HasAReservedIdentifierTypeCode(u16),
	
	/// Resource data for resource record type `DHCID` has reserved digest type code.
	HasReservedDigestTypeCode,
	
	/// Resource data for resource record type `DHCID` has an incorrect digest length (value in tuple).
	HasADigestLengthThatIsIncorrectForTheMatchingType(usize),
}

impl Display for DHCIDHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DHCIDHandleRecordTypeError
{
}
