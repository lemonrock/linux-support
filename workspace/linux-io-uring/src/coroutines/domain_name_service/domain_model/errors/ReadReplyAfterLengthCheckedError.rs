// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read reply after length checked error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ReadReplyAfterLengthCheckedError<E: error::Error>
{
	/// Message header.
	MessageHeader(MessageHeaderError),
	
	/// Section.
	Section(SectionError<E>),
	
	/// A message, once parsed, had bytes remaining in the TCP buffer.
	MessageHadUnparsedBytesAtEnd(MessageIdentifier),
}

impl<E: error::Error> Display for ReadReplyAfterLengthCheckedError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: 'static + error::Error> error::Error for ReadReplyAfterLengthCheckedError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ReadReplyAfterLengthCheckedError::*;
		
		match self
		{
			&MessageHeader(ref error) => Some(error),
			
			&Section(ref error) => Some(error),
			
			_ => None,
		}
	}
}

impl<E: error::Error> From<MessageHeaderError> for ReadReplyAfterLengthCheckedError<E>
{
	#[inline(always)]
	fn from(value: MessageHeaderError) -> Self
	{
		ReadReplyAfterLengthCheckedError::MessageHeader(value)
	}
}

impl<E: error::Error> From<SectionError<E>> for ReadReplyAfterLengthCheckedError<E>
{
	#[inline(always)]
	fn from(value: SectionError<E>) -> Self
	{
		ReadReplyAfterLengthCheckedError::Section(value)
	}
}
