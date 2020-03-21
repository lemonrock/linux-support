// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the unique address of a PCI device in a system, such as an individual ethernet port (connector).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
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

impl TryFrom<&str> for PciDeviceAddress
{
	type Error = PciDeviceAddressStringParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error>
	{
		// Number of bytes in a PCI address string formatted as `XXXX:XX:XX.XX`.
		const NumberOfBytesInPciAddressString: usize = 13;

		let mut match_count = 0;

		let length = value.len();
		if length != NumberOfBytesInPciAddressString
		{
			return Err(PciDeviceAddressStringParseError::LengthIsWrong { length });
		}

		let mut split = value.split(|character|
		{
			match match_count
			{
				0 | 1 =>
				{
					match_count += 1;
					character == ':'
				},

				2 =>
				{
					match_count += 1;
					character == '.'
				},

				_ => false,
			}
		});

		let domain = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoDomain),
			Some(value) =>
			{
				match u16::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDomain { value: value.to_owned(), cause }),
					Ok(value) => value,
				}
			}
		};

		let bus = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoBus),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseBus { value: value.to_owned(), cause }),
					Ok(value) => value,
				}
			}
		};

		let devid = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoDeviceIdentifier),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseDeviceIdentifier { value: value.to_owned(), cause }),
					Ok(value) => if value > 31
					{
						return Err(PciDeviceAddressStringParseError::DeviceNumberExceeds5BitValue { value })
					}
					else
					{
						value
					},
				}
			}
		};

		let function = match split.next()
		{
			None => return Err(PciDeviceAddressStringParseError::NoFunction),
			Some(value) =>
			{
				match u8::from_str_radix(value, 16)
				{
					Err(cause) => return Err(PciDeviceAddressStringParseError::CouldNotParseFunction { value: value.to_owned(), cause }),
					Ok(value) => if value > 15
					{
						return Err(PciDeviceAddressStringParseError::FunctionExceeds4BitValue { value })
					}
					else
					{
						value
					},
				}
			}
		};

		Ok
		(
			Self
			{
				domain,
				bus,
				devid,
				function
			}
		)
	}
}
