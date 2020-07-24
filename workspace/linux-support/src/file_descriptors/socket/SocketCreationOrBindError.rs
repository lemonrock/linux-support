// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error that can occur during creation or binding of a socket instance.
#[derive(Debug)]
pub enum SocketCreationOrBindError
{
	/// Creation.
	Creation(CreationError),
	
	/// Binding.
	SocketBind(SocketBindError)
}

impl Display for SocketCreationOrBindError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SocketCreationOrBindError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::SocketCreationOrBindError::*;
		
		match self
		{
			&Creation(ref error) => Some(error),
			&SocketBind(ref error) => Some(error),
		}
	}
}

impl From<CreationError> for SocketCreationOrBindError
{
	#[inline(always)]
	fn from(value: CreationError) -> Self
	{
		SocketCreationOrBindError::Creation(value)
	}
}

impl From<SocketBindError> for SocketCreationOrBindError
{
	#[inline(always)]
	fn from(value: SocketBindError) -> Self
	{
		SocketCreationOrBindError::SocketBind(value)
	}
}
