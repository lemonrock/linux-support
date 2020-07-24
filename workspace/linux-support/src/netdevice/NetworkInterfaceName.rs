// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a network interface name, such as `eth0`.
///
/// Relies on the fact that `IF_NAMESIZE` and `IFNAMSIZ` are the same and both are the same as `TASK_COMM_LEN`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkInterfaceName(ObjectName);

impl Display for NetworkInterfaceName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<ObjectName> for NetworkInterfaceName
{
	#[inline(always)]
	fn from(value: ObjectName) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName> for NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> ObjectName
	{
		self.0
	}
}

impl<'a> Into<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]> for &'a NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.to_object_name()
	}
}

impl Into<[c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]> for NetworkInterfaceName
{
	#[inline(always)]
	fn into(self) -> [c_char; ObjectName::MaximumObjectNameLengthIncludingAsciiNull]
	{
		self.0.into_object_name()
	}
}

impl Deref for NetworkInterfaceName
{
	type Target = ObjectName;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl TryFrom<NetworkInterfaceIndex> for NetworkInterfaceName
{
	type Error = NetworkInterfaceIndexToNetworkInterfaceNameError;
	
	#[inline(always)]
	fn try_from(value: NetworkInterfaceIndex) -> Result<Self, Self::Error>
	{
		Self::try_from_network_interface_index(value)?.ok_or(NetworkInterfaceIndexToNetworkInterfaceNameError::DoesNotExistAsAnInterface)
	}
}

impl NetworkInterfaceName
{
	/// Tries to get the network interface name.
	#[inline(always)]
	pub fn try_from_network_interface_index(value: NetworkInterfaceIndex) -> Result<Option<Self, NetworkDeviceInputOutputControlError<ObjectNameFromBytesError>>>
	{
		NetworkDeviceSocketFileDescriptor::network_interface_index_to_network_interface_name(value)
	}
}
