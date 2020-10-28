// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended DNS 'OPT' record error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ExtendedDnsError
{
	/// More than one resource record of the psuedo-type `OPT` is present in the additional record section.
	MoreThanOneExtendedDnsOptResourceRecord,
	
	/// The OPT extended error code bits are non-zero.
	ResponseCode(ExtendedDnsResponseCodeError),
	
	/// A resource record of the psuedo-type `OPT` has too long a record name.
	ExtendedDnsOptRecordNameTooLong,
	
	/// A resource record of the psuedo-type `OPT` is present with a name other than ''.
	ExtendedDnsOptRecordNameNotRoot,
	
	/// An unsupported EDNS version; unsupported version in tuple.
	UnsupportedExtendedDnsVersion(NonZeroU8),
	
	/// EDNS(0) Option field has a length less than 4.
	ExtendedDnsOptionTooShort,
	
	/// EDNS(0) Option code was in the reserved set (0, 65001-65534 and 65535); code is in tuple.
	///
	/// Code 4 is ignored as the draft it pertains sees occasionaly progress; it might come into being.
	ExtendedDnsOptionCodeWasReserved((u8, u8)),
	
	/// EDNS(0) Option length field indicates an option data field whose length would exceed that remaining in the resource data of the `OPT` resource record.
	ExtendedDnsOptionDataOverflows,
	
	/// Some EDNS(0) options must only be set in a request, or be the result of sending a request
	ExtendedDnsOptionMustOnlyBeSetInARequest((u8, u8)),
	
	/// Resource data length overflows the space available.
	ResourceDataLengthOverflows(ResourceDataLengthOverflowsError),
	
	/// Response did not have the EDNS(0) DNSSEC OK (`DO`) bit set.
	///
	/// NOTE: This behaviour violates RFC 6840, Section 5.6, Setting the DO Bit on Replies.
	ResponseIgnoredDnsSec,
}

impl Display for ExtendedDnsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ExtendedDnsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ExtendedDnsError::*;
		
		match self
		{
			&ResponseCode(ref error) => Some(error),
			
			&ResourceDataLengthOverflows(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl From<ExtendedDnsResponseCodeError> for ExtendedDnsError
{
	#[inline(always)]
	fn from(value: ExtendedDnsResponseCodeError) -> Self
	{
		ExtendedDnsError::ResponseCode(value)
	}
}

impl From<ResourceDataLengthOverflowsError> for ExtendedDnsError
{
	#[inline(always)]
	fn from(value: ResourceDataLengthOverflowsError) -> Self
	{
		ExtendedDnsError::ResourceDataLengthOverflows(value)
	}
}
