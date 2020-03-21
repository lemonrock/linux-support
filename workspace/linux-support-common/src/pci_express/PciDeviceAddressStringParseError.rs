// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

/// PCI device address string parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PciDeviceAddressStringParseError
{
	#[allow(missing_docs)]
	LengthIsWrong
	{
		/// Incorrect length.
		length: usize
	},

	#[allow(missing_docs)]
	NoDomain,

	/// Could not parse domain as u16.
	#[allow(missing_docs)]
	CouldNotParseDomain
	{
		/// Value.
		value: String,

		/// Cause.
		cause: ParseIntError,
	},

	#[allow(missing_docs)]
	NoBus,

	/// Could not parse bus as u8.
	#[allow(missing_docs)]
	CouldNotParseBus
	{
		/// Value.
		value: String,

		/// Cause.
		cause: ParseIntError,
	},

	#[allow(missing_docs)]
	NoDeviceIdentifier,

	/// Could not parse pci_device_identifier as u8
	CouldNotParseDeviceIdentifier
	{
		/// Value.
		value: String,

		/// Cause.
		cause: ParseIntError,
	},

	/// Parsed device number exceeds 5-bit value (ie is 32 or more)
	DeviceNumberExceeds5BitValue
	{
		/// Value.
		value: u8,
	},

	#[allow(missing_docs)]
	NoFunction,

	/// Could not parse function as u8.
	#[allow(missing_docs)]
	CouldNotParseFunction
	{
		/// Value.
		value: String,

		/// Cause.
		cause: ParseIntError,
	},

	/// Parsed function exceeds 4-bit value (ie is 16 or more).
	FunctionExceeds4BitValue
	{
		/// Value.
		value: u8,
	},
}

impl Display for PciDeviceAddressStringParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<PciDeviceAddressStringParseError as Debug>::fmt(self, f)
	}
}

impl error::Error for PciDeviceAddressStringParseError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::PciDeviceAddressStringParseError::*;

		match self
		{
			&LengthIsWrong { .. } => None,

			&NoDomain => None,

			&CouldNotParseDomain { ref cause, .. } => Some(cause),

			&NoBus => None,

			&CouldNotParseBus { ref cause, .. } => Some(cause),

			&NoDeviceIdentifier => None,

			&CouldNotParseDeviceIdentifier { ref cause, .. } => Some(cause),

			&DeviceNumberExceeds5BitValue { .. } => None,

			&NoFunction => None,

			&CouldNotParseFunction { ref cause, .. } => Some(cause),

			&FunctionExceeds4BitValue { .. } => None,
		}
	}
}
