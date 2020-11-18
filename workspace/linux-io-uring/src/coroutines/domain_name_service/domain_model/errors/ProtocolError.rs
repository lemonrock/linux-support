// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A protocol error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ProtocolError<E: 'static + error::Error>
{
	/// Message length.
	MessageLength(MessageLengthError),
	
	/// Read reply after length checked.
	ReadReplyAfterLengthChecked(ReadReplyAfterLengthCheckedError<E>),
	
	/// Cache store.
	CacheStore(CacheStoreError),
}

impl<E: error::Error> Display for ProtocolError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: 'static + error::Error> error::Error for ProtocolError<E>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProtocolError::*;
		
		match self
		{
			&MessageLength(ref error) => Some(error),
			
			&ReadReplyAfterLengthChecked(ref error) => Some(error),
			
			&CacheStore(ref error) => Some(error),
		}
	}
}

impl<E: error::Error> From<MessageLengthError> for ProtocolError<E>
{
	#[inline(always)]
	fn from(value: MessageLengthError) -> Self
	{
		ProtocolError::MessageLength(value)
	}
}

impl<E: error::Error> From<CacheStoreError> for ProtocolError<E>
{
	#[inline(always)]
	fn from(value: CacheStoreError) -> Self
	{
		ProtocolError::CacheStore(AnsweredError)
	}
}
