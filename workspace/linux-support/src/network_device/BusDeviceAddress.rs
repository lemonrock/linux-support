// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a bus device address (eg a PCI address separated by `:`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BusDeviceAddress(ObjectName32);

impl Display for BusDeviceAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<ObjectName32> for BusDeviceAddress
{
	#[inline(always)]
	fn from(value: ObjectName32) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName32> for BusDeviceAddress
{
	#[inline(always)]
	fn into(self) -> ObjectName32
	{
		self.0
	}
}

impl<'a> Into<[c_char; ObjectName32::MaximumLengthIncludingAsciiNull]> for &'a BusDeviceAddress
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName32::MaximumLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName32::MaximumLengthIncludingAsciiNull]> for BusDeviceAddress
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName32::MaximumLengthIncludingAsciiNull]
	{
		self.0.into_object_name()
	}
}

impl Deref for BusDeviceAddress
{
	type Target = ObjectName32;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'a> TryFrom<&'a NetworkInterfaceName> for BusDeviceAddress
{
	type Error = NetworkInterfaceNameToSomethingError<ObjectNameFromBytesError>;
	
	#[inline(always)]
	fn try_from(value: &'a NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		Self::try_from(value.clone())
	}
}

impl TryFrom<NetworkInterfaceName> for BusDeviceAddress
{
	type Error = NetworkInterfaceNameToSomethingError<ObjectNameFromBytesError>;
	
	#[inline(always)]
	fn try_from(value: NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		Self::try_from_network_interface_name(value)?.ok_or(NetworkInterfaceNameToSomethingError::DoesNotExistAsAnInterface)
	}
}

impl BusDeviceAddress
{
	/// Tries to get the network interface name.
	#[inline(always)]
	pub fn try_from_network_interface_name(value: NetworkInterfaceName) -> Result<Option<Self>, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>
	{
		NetworkDeviceSocketFileDescriptor::bus_device_address(value)
	}
}
