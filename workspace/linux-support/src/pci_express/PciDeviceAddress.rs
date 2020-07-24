// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a PCI device in a system, such as an individual ethernet port (connector).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PciDeviceAddress
{
	domain: u16,
	bus: u8,
	devid: u8,
	function: u8,
}

impl Into<String> for PciDeviceAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		(&self).into()
	}
}

impl<'a> Into<String> for &'a PciDeviceAddress
{
	#[inline(always)]
	fn into(self) -> String
	{
		format!("{:04x}:{:02x}:{:02x}.{:01x}", self.domain, self.bus, self.devid, self.function)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for PciDeviceAddress
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let string: String = self.into();
		let mut bytes = string.into_bytes();
		bytes.push(b'\n');
		Cow::from(bytes)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a PciDeviceAddress
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let string: String = self.into();
		let mut bytes = string.into_bytes();
		bytes.push(b'\n');
		Cow::from(bytes)
	}
}

impl<'a> TryFrom<&'a NetworkInterfaceName> for PciDeviceAddress
{
	type Error = PciDeviceAddressFromNetworkInterfaceNameError;

	#[inline(always)]
	fn try_from(network_interface_name: &'a NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		let bus_device_address = BusDeviceAddress::try_from(network_interface_name)?;
		Self::try_from(bus_device_address)
	}
}

impl TryFrom<NetworkInterfaceName> for PciDeviceAddress
{
	type Error = PciDeviceAddressFromNetworkInterfaceNameError;

	#[inline(always)]
	fn try_from(network_interface_name: NetworkInterfaceName) -> Result<Self, Self::Error>
	{
		let bus_device_address = BusDeviceAddress::try_from(network_interface_name)?;
		Self::try_from(bus_device_address)
	}
}

impl<'a> TryFrom<&'a BusDeviceAddress> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: &'a BusDeviceAddress) -> Result<Self, Self::Error>
	{
		Self::try_from(value.deref())
	}
}

impl TryFrom<BusDeviceAddress> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: BusDeviceAddress) -> Result<Self, Self::Error>
	{
		Self::try_from(value.deref())
	}
}

impl TryFrom<String> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: String) -> Result<Self, Self::Error>
	{
		Self::try_from(value.into_bytes())
	}
}

impl<'a> TryFrom<&'a str> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: &'a str) -> Result<Self, Self::Error>
	{
		Self::try_from(value.as_bytes())
	}
}

impl TryFrom<Box<[u8]>> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error>
	{
		Self::try_from(&value[..])
	}
}

impl TryFrom<Vec<u8>> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>
	{
		Self::try_from(&value[..])
	}
}

impl<'a> TryFrom<&'a [c_char]> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;
	
	#[inline(always)]
	fn try_from(value: &'a [c_char]) -> Result<Self, Self::Error>
	{
		let value: &'a [u8] = unsafe { transmute(value) };
		Self::try_from(value)
	}
}

impl<'a> TryFrom<&'a [u8]> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		// Number of bytes in a PCI address string formatted as `XXXX:XX:XX.XX`.
		const NumberOfBytesInPciAddressString: usize = 13;
		let length = value.len();
		if length != NumberOfBytesInPciAddressString
		{
			return Err(PciDeviceAddressStringParseError::LengthIsWrong { length });
		}

		let mut match_count = 0;
		let mut split = value.splitn(4, |character|
		{
			match match_count
			{
				0 | 1 =>
				{
					match_count += 1;
					*character == b':'
				},

				2 =>
				{
					match_count += 1;
					*character == b'.'
				},

				_ => unreachable!("Should not be possible"),
			}
		});

		use self::PciDeviceAddressStringParseError::*;

		Ok
		(
			Self
			{
				domain:
				{
					let value = split.next().ok_or(NoDomain)?;
					u16::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseDomain { value: value.to_owned(), cause})?
				},
				bus:
				{
					let value = split.next().ok_or(NoDeviceIdentifier)?;
					u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseBus { value: value.to_owned(), cause})?
				},
				devid:
				{
					let value = split.next().ok_or(NoDeviceIdentifier)?;
					let value = u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseDeviceIdentifier { value: value.to_owned(), cause})?;
					if unlikely!(value > 31)
					{
						return Err(DeviceNumberExceeds5BitValue { value })
					}
					value
				},
				function:
				{

					let value = split.next().ok_or(NoFunction)?;
					let value = u8::parse_hexadecimal_number_upper_or_lower_case(value).map_err(|cause| CouldNotParseFunction { value: value.to_owned(), cause})?;
					if unlikely!(value > 15)
					{
						return Err(FunctionExceeds4BitValue { value })
					}
					value
				},
			}
		)
	}
}
