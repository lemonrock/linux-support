// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during creation of a socket listener.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NewSocketClientError
{
	/// Creation.
	Creation(CreationError),

	/// Connect.
	Connect(SocketConnectError),
}

impl Display for NewSocketClientError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<NewSocketClientError as Debug>::fmt(self, f)
	}
}

impl error::Error for NewSocketClientError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NewSocketClientError::*;

		match self
		{
			&Creation(ref error) => Some(error),
			&Connect(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for NewSocketClientError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		NewSocketClientError::Creation(error)
	}
}

impl From<SocketConnectError> for NewSocketClientError
{
	#[inline(always)]
	fn from(error: SocketConnectError) -> Self
	{
		NewSocketClientError::Connect(error)
	}
}
