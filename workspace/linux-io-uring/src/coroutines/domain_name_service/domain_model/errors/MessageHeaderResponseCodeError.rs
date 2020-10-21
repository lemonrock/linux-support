// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message header response code error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MessageHeaderResponseCodeError
{
	/// We produced a bad query; we didn't.
	WasFormatError,
	
	/// This is NOT returned for data that failed validation when using DNSSEC.
	WasServerFailure,
	
	/// This should not occur.
	WasNonExistentDomainForANonAuthoritativeServer,
	
	/// Rare; indicates a server does not support a particular DNS OpCode.
	///
	/// Since every server should support the `Query` OpCde, this is pretty fatal.
	///
	/// Can occur also when using a server that doesn't support DNSSEC.
	WasNotImplemented,
	
	/// Permission denied, effectively.
	WasRefused,
	
	/// Message response code should not be dynamic DNS associated.
	ShouldNotBeDynamicDnsAssociated(u8),
	
	/// Message response code should not be DNS stateful operation type not implemented.
	ShouldNotBeDnsStatefulOperationsTypeNotImplemented,
	
	/// Message response code unassigned.
	Unassigned(u8),
}

impl Display for MessageHeaderResponseCodeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MessageHeaderResponseCodeError
{
}
