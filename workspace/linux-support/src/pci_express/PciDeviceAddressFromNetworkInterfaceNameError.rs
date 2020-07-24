// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PciDeviceAddressFromNetworkInterfaceNameError
{
	/// Could not get it.
	NetworkInterfaceNameToSomething(NetworkInterfaceNameToSomethingError<EthToolStringFromBytesError>),
	
	/// Could not parse it.
	PciDeviceAddressStringParse(PciDeviceAddressStringParseError),
}

impl Display for PciDeviceAddressFromNetworkInterfaceNameError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for PciDeviceAddressFromNetworkInterfaceNameError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::PciDeviceAddressFromNetworkInterfaceNameError::*;
		
		match self
		{
			&NetworkInterfaceNameToSomething(ref error) => Some(error),
			
			&PciDeviceAddressStringParse(ref error) => Some(error),
		}
	}
}

impl From<NetworkInterfaceNameToSomethingError<EthToolStringFromBytesError>> for PciDeviceAddressFromNetworkInterfaceNameError
{
	#[inline(always)]
	fn from(value: NetworkInterfaceNameToSomethingError<EthToolStringFromBytesError>) -> Self
	{
		PciDeviceAddressFromNetworkInterfaceNameError::NetworkInterfaceNameToSomething(value)
	}
}

impl From<PciDeviceAddressStringParseError> for PciDeviceAddressFromNetworkInterfaceNameError
{
	#[inline(always)]
	fn from(value: PciDeviceAddressStringParseError) -> Self
	{
		PciDeviceAddressFromNetworkInterfaceNameError::PciDeviceAddressStringParse(value)
	}
}
