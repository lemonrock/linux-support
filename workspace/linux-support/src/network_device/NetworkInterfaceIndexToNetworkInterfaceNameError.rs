// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors that can occur when converting an network interface name to an index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkInterfaceIndexToNetworkInterfaceNameError
{
	/// Control operation failed.
	NetworkDeviceInputOutputControl(NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>),
	
	/// Does not exist as an interface.
	DoesNotExistAsAnInterface,
}

impl Display for NetworkInterfaceIndexToNetworkInterfaceNameError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for NetworkInterfaceIndexToNetworkInterfaceNameError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkInterfaceIndexToNetworkInterfaceNameError::*;

		match self
		{
			&NetworkDeviceInputOutputControl(ref error) => Some(error),

			&DoesNotExistAsAnInterface => None,
		}
	}
}

impl From<NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>> for NetworkInterfaceIndexToNetworkInterfaceNameError
{
	#[inline(always)]
	fn from(value: NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>) -> Self
	{
		NetworkInterfaceIndexToNetworkInterfaceNameError::NetworkDeviceInputOutputControl(value)
	}
}
