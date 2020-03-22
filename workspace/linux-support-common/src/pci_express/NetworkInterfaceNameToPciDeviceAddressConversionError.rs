// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors that can occur when converting an network interface name to a PCI device address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkInterfaceNameToPciDeviceAddressConversionError
{
	/// Name failed to convert.
	Name(NetworkInterfaceNameToIndexConversionError),

	/// Index failed to convert.
	Index(ConvertNetworkInterfaceIndexToPciDeviceAddressError),
}

impl From<NetworkInterfaceNameToIndexConversionError> for NetworkInterfaceNameToPciDeviceAddressConversionError
{
	#[inline(always)]
	fn from(value: NetworkInterfaceNameToIndexConversionError) -> Self
	{
		NetworkInterfaceNameToPciDeviceAddressConversionError::Name(value)
	}
}

impl From<ConvertNetworkInterfaceIndexToPciDeviceAddressError> for NetworkInterfaceNameToPciDeviceAddressConversionError
{
	#[inline(always)]
	fn from(value: ConvertNetworkInterfaceIndexToPciDeviceAddressError) -> Self
	{
		NetworkInterfaceNameToPciDeviceAddressConversionError::Index(value)
	}
}

impl Display for NetworkInterfaceNameToPciDeviceAddressConversionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<NetworkInterfaceNameToPciDeviceAddressConversionError as Debug>::fmt(self, f)
	}
}

impl error::Error for NetworkInterfaceNameToPciDeviceAddressConversionError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkInterfaceNameToPciDeviceAddressConversionError::*;

		match self
		{
			&Name(ref cause) => Some(cause),

			&Index(ref cause) => Some(cause),
		}
	}
}
