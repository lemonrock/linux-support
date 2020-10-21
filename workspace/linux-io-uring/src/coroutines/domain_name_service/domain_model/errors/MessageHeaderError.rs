// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message header error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MessageHeaderError
{
	/// Response was a query.
	ResponseWasAQuery(MessageIdentifier),
	
	/// An unexpected reply message.
	///
	/// `(Actual, Expected)`.
	UnexpectedReplyMessage(MessageIdentifier, MessageIdentifier),
	
	/// Response does not contain exactly one question.
	DoesNotContainExactlyOneQuestion(MessageIdentifier, u16),
	
	/// Response was authoritative (`AA` bit is set) but also has the authenticated data (`AD`) bit set; this is not possible, as an authoritative name server can not authenticate its own signatures!
	ResponseWasAuthoritativeButHasTheAuthenticatedDataBitSet,
	
	/// Message header response opcode error.
	ResponseOpcode(MessageIdentifier, MessageHeaderResponseOpcodeError),
	
	/// Message header response code error.
	ResponseCode(MessageIdentifier, MessageHeaderResponseCodeError),
	
	/// Response used reserved header bits (`Z`).
	UsedReservedHeaderBits(MessageIdentifier),
	
	/// Response is truncated (`TC`).
	IsTruncated(MessageIdentifier),
	
	/// Response failed to copy the recursion desired (`RD`) bit.
	FailedToCopyRecursionDesiredBit(MessageIdentifier),
	
	/// Response failed to copy the checking disabled (`CD`) bit.
	FailedToCopyCheckingDisabledBit(MessageIdentifier),
}

impl Display for MessageHeaderError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MessageHeaderError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MessageHeaderError::*;
		
		match self
		{
			&ResponseOpcode(_message_identifier, ref error) => Some(error),
			
			&ResponseCode(_message_identifier, ref error) => Some(error),
			
			_ => None,
		}
	}
}
