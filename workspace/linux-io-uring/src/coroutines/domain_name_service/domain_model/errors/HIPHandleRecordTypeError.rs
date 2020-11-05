// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle `HIP` record type error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HIPHandleRecordTypeError
{
	/// Resource data for resource record type `HIP` has an incorrect length (value in tuple).
	HasAnIncorrectLength(usize),
	
	/// Public key error.
	IpsecLikePublicKeyHandleRecordType(IpsecLikePublicKeyHandleRecordTypeError),
	
	/// First rendezvous server domain name.
	FirstRendezvousServerDomainName(ParsedNameParserError),
	
	/// Subsequent rendezvous server domain name.
	SubsequentRendezvousServerDomainName(ParsedNameParserError),
	
	/// Duplicate rendezvous server domain name.
	DuplicateRendezvousServerDomainName,
}

impl Display for HIPHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for HIPHandleRecordTypeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::HIPHandleRecordTypeError::*;
		
		match self
		{
			&IpsecLikePublicKeyHandleRecordType(ref error) => Some(error),
			
			&FirstRendezvousServerDomainName(ref error) => Some(error),
			
			&SubsequentRendezvousServerDomainName(ref error) => Some(error),
			
			_ => None,
		}
	}
}
