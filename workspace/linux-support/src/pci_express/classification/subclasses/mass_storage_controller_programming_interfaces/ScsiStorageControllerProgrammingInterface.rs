// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// SCSI Storage Controller Programming Interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum ScsiStorageControllerProgrammingInterface
{
	/// Vendor-specific.
	VendorSpecific = 0x00,

	/// SCSI storage device (e.g., hard disk drive (HDD), solid state drive (SSD), or RAID controller) - SCSI over PCI Express (SOP) target port using PCIExpressQueuing Interface (PQI).
	StorageDevice = 0x11,

	/// SCSI controller (i.e., host bus adapter) - SCSI over PCI Express (SOP) target port using PCI Express Queuing Interface (PQI).
	Controller = 0x12,

	/// SCSI storage device and SCSI controller - SCSI over PCI Express (SOP) target port using PCI Express Queuing Interface (PQI).
	StorageDeviceAndController = 0x13,

	/// SCSI storage device - SCSI over PCI Express (SOP) target port using the queueing interface portion of the NVM Express interface.
	StorageDeviceUsingNvmExpress = 0x21,
}

impl ScsiStorageControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::ScsiStorageControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(VendorSpecific),
			0x11 => Some(StorageDevice),
			0x12 => Some(Controller),
			0x13 => Some(StorageDeviceAndController),
			0x21 => Some(StorageDeviceUsingNvmExpress),
			_ => None,
		}
	}
}
