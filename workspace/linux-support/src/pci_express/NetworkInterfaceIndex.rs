// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a one-based network interface.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkInterfaceIndex(NonZeroU32);

impl<'a> TryFrom<&'a NetworkInterfaceName> for NetworkInterfaceIndex
{
	type Error = NetworkInterfaceNameToIndexConversionError;

	fn try_from(network_interface_name: &'a NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		use self::NetworkInterfaceNameToIndexConversionError::*;

		if network_interface_name.is_empty()
		{
			return Err(ZeroSized)
		}

		if network_interface_name.len() > IF_NAMESIZE
		{
			return Err(TooLong)
		}

		let value = match CString::new(network_interface_name.0.as_str())
		{
			Err(error) => return Err(InvalidCString(error)),

			Ok(value) => value,
		};

		match unsafe { if_nametoindex(value.as_ptr()) }
		{
			0 => Err(DoesNotExistAsAnInterface),

			one_based_index => Ok(Self(unsafe { NonZeroU32::new_unchecked(one_based_index) })),
		}
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
