// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI device identifier.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PciDeviceIdentifier(u16);

impl From<u16> for PciDeviceIdentifier
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		PciDeviceIdentifier(value)
	}
}

impl Into<u16> for PciDeviceIdentifier
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self.0
	}
}

impl FromBytes for PciDeviceIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let raw_value = u16::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(bytes, size_of::<u16>() * 2)?;
		if unlikely!(raw_value == Self::AnyOrInvalidRaw)
		{
			Err(ParseNumberError::WasMaximum)
		}
		else
		{
			Ok(Self(raw_value))
		}
	}
}

impl PciDeviceIdentifier
{
	const AnyOrInvalidRaw: u16 = 0xFFFF;
	
	/// Any or invalid.
	pub const AnyOrInvalid: PciDeviceIdentifier = PciDeviceIdentifier(Self::AnyOrInvalidRaw);

	/// New, unchecked apart from a static assertion.
	#[inline(always)]
	pub const fn new_unchecked(pci_device_identifier: u16) -> Self
	{
		cfn_assert_ne!(pci_device_identifier, Self::AnyOrInvalidRaw);
		Self(pci_device_identifier)
	}
	
	/// New.
	#[inline(always)]
	pub fn new(pci_device_identifier: u16) -> Option<Self>
	{
		if pci_device_identifier == Self::AnyOrInvalidRaw
		{
			None
		}
		else
		{
			Some(PciDeviceIdentifier(pci_device_identifier))
		}
	}
	
	/// Is this any or invalid?
	#[inline(always)]
	pub fn is_any_or_invalid(&self) -> bool
	{
		self.0 == Self::AnyOrInvalidRaw
	}
	
	/// Is?
	#[inline(always)]
	pub fn is(&self, other: u16) -> bool
	{
		self.0 == other
	}
}
