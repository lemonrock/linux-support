// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a one-based network interface.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkInterfaceIndex(NonZeroU32);

impl Into<NonZeroU32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn into(self) -> NonZeroU32
	{
		self.0
	}
}

impl Into<i32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn into(self) -> i32
	{
		let x: u32 = self.0.into();
		x as i32
	}
}

impl From<NonZeroU32> for NetworkInterfaceIndex
{
	#[inline(always)]
	fn from(value: NonZeroU32) -> Self
	{
		Self(value)
	}
}

impl TryFrom<u32> for NetworkInterfaceIndex
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(unsafe { NonZeroU32::new_unchecked(value) }))
		}
	}
}

impl TryFrom<i32> for NetworkInterfaceIndex
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: i32) -> Result<Self, Self::Error>
	{
		if unlikely!(value == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else if unlikely!(value < 0)
		{
			Err(ParseNumberError::OutOfRange)
		}
		else
		{
			Ok(Self(unsafe { NonZeroU32::new_unchecked(value as u32) }))
		}
	}
}

impl<'a> TryFrom<&'a NetworkInterfaceName> for NetworkInterfaceIndex
{
	type Error = NetworkInterfaceNameToSomethingError<ParseNumberError>;
	
	#[inline(always)]
	fn try_from(value: &'a NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		Self::try_from(value.clone())
	}
}

impl TryFrom<NetworkInterfaceName> for NetworkInterfaceIndex
{
	type Error = NetworkInterfaceNameToSomethingError<ParseNumberError>;
	
	#[inline(always)]
	fn try_from(value: NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		use self::NetworkInterfaceNameToSomethingError::*;
		
		Self::try_from_network_interface_name(value).map_err(NetworkDeviceInputOutputControl)?.ok_or(DoesNotExistAsAnInterface)
	}
}

impl NetworkInterfaceIndex
{
	/// Tries to get the network interface index.
	#[inline(always)]
	pub fn try_from_network_interface_name(value: NetworkInterfaceName) -> Result<Option<Self>, NetworkDeviceInputOutputControlError<ParseNumberError>>
	{
		NetworkDeviceSocketFileDescriptor::network_interface_name_to_network_interface_index(value)
	}
}
