// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DNS protocol error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DnsProtocolError
{
	/// A message is shorter than the minimum.
	MessageIsTooShort(usize),
	
	/// A message is longer than the maximum.
	MessageIsTooLong(usize),
	
	/// An unexpected reply message.
	UnexpectedReplyMessage(MessageIdentifier),

	/// A message, once parsed, had bytes remaining in the TCP buffer.
	MessageHadUnparsedBytesAtEnd(MessageIdentifier),

	/// Response was a query.
	ResponseWasAQuery,

	/// Invalid response opcode.
	InvalidResponseOpcode(u8),

	/// Response `QNAME` did not match.
	ResponseWasForADifferentName,

	/// Response `QTYPE` did not match.
	ResponseWasForADifferentDataType,

	/// Unassigned response opcode.
	UnassignedResponseOpcode(u8),

	/// Response used reserved header bits (`Z`).
	ResponseUsedReservedHeaderBits,

	/// Response is truncated (`TC`).
	ResponseIsTruncated,

	/// Response failed to copy the recursion desired (`RD`) bit.
	ResponseFailedToCopyRecursionDesiredBit,

	/// Response failed to copy the checking disabled (`CD`) bit.
	ResponseFailedToCopyCheckingDisabledBit,

	/// Response was authoritative (`AA` bit is set) but also has the authenticated data (`AD`) bit set; this is not possible, as an authoritative name server can not authenticate its own signatures!
	ResponseWasAuthoritativeButHasTheAuthenticatedDataBitSet,

	/// We produced a bad query; we didn't.
	MessageResponseCodeWasFormatError,

	/// This is NOT returned for data that failed validation when using DNSSEC.
	MessageResponseCodeWasServerFailure,

	/// This should not occur.
	MessageResponseCodeWasNonExistentDomainForANonAuthoritativeServer,

	/// Rare; indicates a server does not support a particular DNS OpCode.
	///
	/// Since every server should support the `Query` OpCde, this is pretty fatal.
	///
	/// Can occur also when using a server that doesn't support DNSSEC.
	MessageResponseCodeWasNotImplemented,

	/// Permission denied, effectively.
	MessageResponseCodeWasRefused,

	/// Message response code should not be dynamic DNS associated.
	MessageResponseCodeShouldNotBeDynamicDnsAssociated(u8),

	/// Message response code should not be DNS stateful operation type not implemented.
	MessageResponseCodeShouldNotBeDnsStatefulOperationsTypeNotImplemented,

	/// Message response code unassigned.
	MessageResponseCodeUnassigned(u8),

	/// Response does not contain exactly one question.
	ResponseDoesNotContainExactlyOneQuestion(u16),
	
	/// DNS `QCLASS` is reserved (including for private use), unassigned or obsolete (ie Chaos or Hesiod).
	///
	/// Tuple contains value.
	///
	/// See [IANA](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-2) and RFC 6895 for further details.
	ClassIsReservedUnassignedOrObsolete(BigEndianU16),
}

impl Display for DnsProtocolError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for DnsProtocolError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::DnsProtocolError::*;
		
		match self
		{
			&Authority(ref error) => Some(error),
			
			_ => None,
		}
	}
}
