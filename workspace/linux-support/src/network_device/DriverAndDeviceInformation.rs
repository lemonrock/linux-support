// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Driver and device information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DriverAndDeviceInformation
{
	/// Driver name.
	pub driver_name: ObjectName32,
	
	/// If `Some` will not be empty.
	///
	/// As of 2020, most drivers now default to `UTS_RELEASE` (the value of `LinuxKernelVersion.release) and ethtool 's `ioctl()` interface ensures the driver version is populated.
	///
	/// So unlikely to be `None`.
	pub driver_version: Option<ObjectName32>,
	
	/// If `Some` will not be empty.
	///
	/// May not be a PCI address.
	///
	/// Usually populated.
	pub device_bus_device_address: Option<BusDeviceAddress>,
	
	/// If `Some` will not be empty.
	///
	/// Not usually populated.
	pub device_firmware_version: Option<ObjectName32>,
	
	/// Expansion ROM version.
	///
	/// If `Some` will not be empty.
	///
	/// Not usually populated.
	pub device_expansion_eeprom_version: Option<ObjectName32>,
	
	/// Expansion EEPROM.
	#[serde(skip)] pub device_eeprom_blob_size_in_bytes: Option<NonZeroU32>,
	
	/// Registers.
	#[serde(skip)] pub device_registers_blob_size_in_bytes: Option<NonZeroU32>,
}
