// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during creation of a socket listener.
#[derive(Debug)]
pub enum NewSocketClientListenerError
{
	/// Creation.
	Creation(CreationError),

	/// Bind.
	Bind(SocketBindError),

	/// Listen.
	Listen(SocketListenError),
	
	/// Connect.
	Connect(SocketConnectError),
}

impl Display for NewSocketClientListenerError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NewSocketClientListenerError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NewSocketClientListenerError::*;

		match self
		{
			&Creation(ref error) => Some(error),
			&Bind(ref error) => Some(error),
			&Listen(ref error) => Some(error),
			&Connect(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for NewSocketClientListenerError
{
	#[inline(always)]
	fn from(error: CreationError) -> Self
	{
		NewSocketClientListenerError::Creation(error)
	}
}

impl From<SocketBindError> for NewSocketClientListenerError
{
	#[inline(always)]
	fn from(error: SocketBindError) -> Self
	{
		NewSocketClientListenerError::Bind(error)
	}
}

impl From<SocketListenError> for NewSocketClientListenerError
{
	#[inline(always)]
	fn from(error: SocketListenError) -> Self
	{
		NewSocketClientListenerError::Listen(error)
	}
}

impl From<SocketConnectError> for NewSocketClientListenerError
{
	#[inline(always)]
	fn from(error: SocketConnectError) -> Self
	{
		NewSocketClientListenerError::Connect(error)
	}
}
