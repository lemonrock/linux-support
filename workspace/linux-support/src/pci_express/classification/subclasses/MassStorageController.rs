// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mass storage controllers.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum MassStorageController
{
	/// SCSI.
	ScsiStorageController(ScsiStorageControllerProgrammingInterface),
	
	/// IDE.
	IdeInterface,

	/// Floppy disk.
	FloppyDiskController,
	
	/// IPI bus.
	IpiBusController,
	
	/// RAID bus.
	RaidBusController,
	
	/// ATA (also known as PATA, Parallel ATA).
	AtaController(AtaControllerProgrammingInterface),
	
	/// Serial ATA (SATA).
	SerialAtaController(SerialAtaControllerProgrammingInterface),
	
	/// Serial Attached SCSI (SAS).
	SerialAttachedScsiController(SerialAttachedScsiControllerProgrammingInterface),
	
	/// NVMe.
	NonVolatileMemorySubsystem(NonVolatileMemorySubsystemProgrammingInterface),

	/// Universal Flash Storage (UFS).
	UniversalFlashStorageController(UniversalFlashStorageControllerProgrammingInterface),
	
	/// Other.
	Other,
}

impl MassStorageController
{
	#[inline(always)]
	pub(crate) fn programming_interface(self) -> u8
	{
		use self::MassStorageController::*;

		match self
		{
			ScsiStorageController(programming_interface) => programming_interface as u8,
			AtaController(programming_interface) => programming_interface as u8,
			SerialAtaController(programming_interface) => programming_interface as u8,
			SerialAttachedScsiController(programming_interface) => programming_interface as u8,
			NonVolatileMemorySubsystem(programming_interface) => programming_interface as u8,
			UniversalFlashStorageController(programming_interface) => programming_interface as u8,

			_ => 0x00,
		}
	}

	#[inline(always)]
	pub(crate) fn parse(value: u8, programming_interface: u8) -> Option<Self>
	{
		use self::MassStorageController::*;

		match value
		{
			0x00 => programming_interface!(ScsiStorageController, programming_interface, ScsiStorageControllerProgrammingInterface),
			0x01 => zero_programming_interface!(IdeInterface, programming_interface),
			0x02 => zero_programming_interface!(FloppyDiskController, programming_interface),
			0x03 => zero_programming_interface!(IpiBusController, programming_interface),
			0x04 => zero_programming_interface!(RaidBusController, programming_interface),
			0x05 => programming_interface!(AtaController, programming_interface, AtaControllerProgrammingInterface),
			0x06 => programming_interface!(SerialAtaController, programming_interface, SerialAtaControllerProgrammingInterface),
			0x07 => programming_interface!(SerialAttachedScsiController, programming_interface, SerialAttachedScsiControllerProgrammingInterface),
			0x08 => programming_interface!(NonVolatileMemorySubsystem, programming_interface, NonVolatileMemorySubsystemProgrammingInterface),
			0x09 => programming_interface!(UniversalFlashStorageController, programming_interface, UniversalFlashStorageControllerProgrammingInterface),
			0x80 => zero_programming_interface!(Other, programming_interface),
			_ => None,
		}
	}

	#[inline(always)]
	pub(crate) fn minor(self) -> u8
	{
		use self::MassStorageController::*;

		match self
		{
			ScsiStorageController(_) => 0x00,
			IdeInterface => 0x01,
			FloppyDiskController => 0x02,
			IpiBusController => 0x03,
			RaidBusController => 0x04,
			AtaController(_) => 0x05,
			SerialAtaController(_) => 0x06,
			SerialAttachedScsiController(_) => 0x07,
			NonVolatileMemorySubsystem(_) => 0x08,
			UniversalFlashStorageController(_) => 0x09,
			Other => 0x80,
		}
	}
}
