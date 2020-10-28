// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Miscellaneous errors.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError
{
	/// DNS `QCLASS` is reserved (including for private use), unassigned or obsolete (ie Chaos or Hesiod).
	///
	/// Tuple contains value.
	///
	/// See [IANA](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-2) and RFC 6895 for further details.
	ClassIsReservedUnassignedOrObsolete(DataType, BigEndianU16),
	
	/// Resource data length overflows the space available.
	ResourceDataLengthOverflows(DataType, ResourceDataLengthOverflowsError),
	
	/// A resource record was a duplicate (the same name, data type and resource data).
	DuplicateResourceRecord(DataType),
}

impl Display for ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ValidateClassIsInternetAndGetTimeToLiveAndResourceDataError::*;
		
		match self
		{
			&ResourceDataLengthOverflows(_data_type, ref error) => Some(error),
			
			_ => None,
		}
	}
}
