// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during creation of a socket listener.
#[derive(Debug)]
pub enum NewSocketServerListenerError
{
	/// Creation.
	Creation(CreationError),

	/// Bind.
	Bind(SocketBindError),

	/// Listen.
	Listen(SocketListenError),
}

impl Display for NewSocketServerListenerError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<NewSocketServerListenerError as Debug>::fmt(self, f)
	}
}

impl error::Error for NewSocketServerListenerError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NewSocketServerListenerError::*;

		match self
		{
			&Creation(ref error) => Some(error),
			&Bind(ref error) => Some(error),
			&Listen(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for NewSocketServerListenerError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		NewSocketServerListenerError::Creation(error)
	}
}

impl From<SocketBindError> for NewSocketServerListenerError
{
	#[inline(always)]
	fn from(error: SocketBindError) -> Self
	{
		NewSocketServerListenerError::Bind(error)
	}
}

impl From<SocketListenError> for NewSocketServerListenerError
{
	#[inline(always)]
	fn from(error: SocketListenError) -> Self
	{
		NewSocketServerListenerError::Listen(error)
	}
}
