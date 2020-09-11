// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error when working with an `ioctl()` for a network device (also known as a netdev or network_device).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkDeviceInputOutputControlError<E: error::Error + 'static>
{
	/// Could not create a socket to use with `ioctl()`.
	Creation(CreationError),
	
	/// Error occurring during control operation.
	ControlOperation(E),

	/// Permission defined during control operation.
	PermissionDenied,

	/// Out of memory during control operation.
	OutOfKernelMemory,

	/// Other; used for missing string sets with ethtool, for instance.
	Other(String),
}

impl<E: error::Error + 'static> Display for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<E: error::Error + 'static> error::Error for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self
		{
			&Creation(ref error) => Some(error),
			
			&ControlOperation(ref error) => Some(error),
			
			&PermissionDenied => None,
			
			&OutOfKernelMemory => None,
			
			&Other(..) => None,
		}
	}
}

impl<E: error::Error + 'static> From<CreationError> for NetworkDeviceInputOutputControlError<E>
{
	#[inline(always)]
	fn from(value: CreationError) -> Self
	{
		NetworkDeviceInputOutputControlError::Creation(value)
	}
}

impl From<ObjectNameFromBytesError> for NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>
{
	#[inline(always)]
	fn from(value: ObjectNameFromBytesError) -> Self
	{
		NetworkDeviceInputOutputControlError::ControlOperation(value)
	}
}

impl From<TransmissionQueueLengthOutRangeError> for NetworkDeviceInputOutputControlError<TransmissionQueueLengthOutRangeError>
{
	#[inline(always)]
	fn from(value: TransmissionQueueLengthOutRangeError) -> Self
	{
		NetworkDeviceInputOutputControlError::ControlOperation(value)
	}
}

impl NetworkDeviceInputOutputControlError<Infallible>
{
	#[inline(always)]
	pub(crate) fn map_error<E2: error::Error + 'static>(self) -> NetworkDeviceInputOutputControlError<E2>
	{
		use self::NetworkDeviceInputOutputControlError::*;
		
		match self
		{
			Creation(error) => Creation(error),
			
			ControlOperation(_) => unreachable!(),
			
			PermissionDenied => PermissionDenied,
			
			OutOfKernelMemory => OutOfKernelMemory,
			
			Other(other) => Other(other),
		}
	}
}
