// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error when working with an `ioctl()` for a network device (also known as a netdev or netdevice).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkDeviceInputOutputControlError<E: error::Error>
{
	/// Could not create a socket to use with `ioctl()`.
	Creation(CreationError),
	
	/// Error occurring during control operation.
	ControlOperation(E),
}

impl<E: error::Error> Display for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error> error::Error for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self
		{
			&Creation(ref error) => Some(error),
			
			&ControlOperation(ref error) => Some(error),
		}
	}
}

impl<E: error::Error> From<CreationError> for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn from(value: CreationError) -> Self
	{
		NetworkDeviceInputOutputControlError::Creation(value)
	}
}

impl<E: error::Error> From<E> for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn from(value: E) -> Self
	{
		NetworkDeviceInputOutputControlError::ControlOperation(value)
	}
}
