// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A PCI device class.
///
/// See <https://pcisig.com/sites/default/files/files/PCI_Code-ID_r_1_10__v8_Nov_2017.pdf>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum PciDeviceClass
{
	Legacy(Legacy),
	MassStorageController(MassStorageController),
	NetworkController(NetworkController),
	DisplayController(DisplayController),
	MultimediaController(MultimediaController),
	MemoryController(MemoryController),
	Bridge(Bridge),
	CommunicationController(CommunicationController),
	GenericSystemPeripheral(GenericSystemPeripheral),
	InputDeviceController(InputDeviceController),
	DockingStation(DockingStation),
	Processor(Processor),
	SerialBusController(SerialBusController),
	WirelessController(WirelessController),
	IntelligentInputOutputController(IntelligentInputOutputController),
	SatelliteCommunicationsController(SatelliteCommunicationsController),
	EncryptionController(EncryptionController),
	SignalProcessingController(SignalProcessingController),
	ProcessingAccelerators(ProcessingAccelerators),
	NonEssentialInstrumentation(NonEssentialInstrumentation),

	/// Unofficial.
	Unassigned(Unassigned),
}

impl PciDeviceClass
{
	#[inline(always)]
	pub(crate) fn parse(u24: u32) -> Option<Self>
	{
		#[inline(always)]
		const fn extract_u8(u24: u32, byte_index: u32) -> u8
		{
			let shift = 8 * byte_index;
			((u24 & (0xFF << shift)) >> shift) as u8
		}

		macro_rules! parse
		{
			($class: ident, $u24: ident) =>
			{
				{
					let subclass = extract_u8($u24, 1);
					let program_interface = extract_u8($u24, 0);
					if let Some(subclass) = $class::parse(subclass, program_interface)
					{
						Some(PciDeviceClass::$class(subclass))
					}
					else
					{
						None
					}
				}
			}
		}

		let class = extract_u8(u24, 2);
		match class
		{
			0x00 => parse!(Legacy, u24),
			0x01 => parse!(MassStorageController, u24),
			0x02 => parse!(NetworkController, u24),
			0x03 => parse!(DisplayController, u24),
			0x04 => parse!(MultimediaController, u24),
			0x05 => parse!(MemoryController, u24),
			0x06 => parse!(Bridge, u24),
			0x07 => parse!(CommunicationController, u24),
			0x08 => parse!(GenericSystemPeripheral, u24),
			0x09 => parse!(InputDeviceController, u24),
			0x0A => parse!(DockingStation, u24),
			0x0B => parse!(Processor, u24),
			0x0C => parse!(SerialBusController, u24),
			0x0D => parse!(WirelessController, u24),
			0x0E => parse!(IntelligentInputOutputController, u24),
			0x0F => parse!(SatelliteCommunicationsController, u24),
			0x10 => parse!(EncryptionController, u24),
			0x11 => parse!(SignalProcessingController, u24),
			0x12 => parse!(ProcessingAccelerators, u24),
			0x13 => parse!(NonEssentialInstrumentation, u24),
			0xFF => parse!(Unassigned, u24),

			_ => None,
		}
	}
	
	/// A PCI device (class, subclass, programming interface) as an (u8, (u8, u8)) tuple.
	#[inline(always)]
	#[allow(missing_docs)]
	pub fn major_minor_programming_interface(&self) -> (u8, u8, u8)
	{
		use self::PciDeviceClass::*;
		
		match *self
		{
			Legacy(subclass) => (0x00, subclass.minor(), subclass.programming_interface()),
			MassStorageController(subclass) => (0x01, subclass.minor(), subclass.programming_interface()),
			NetworkController(subclass) => (0x02, subclass.minor(), subclass.programming_interface()),
			DisplayController(subclass) => (0x03, subclass.minor(), subclass.programming_interface()),
			MultimediaController(subclass) => (0x04, subclass.minor(), subclass.programming_interface()),
			MemoryController(subclass) => (0x05, subclass.minor(), subclass.programming_interface()),
			Bridge(subclass) => (0x06, subclass.minor(), subclass.programming_interface()),
			CommunicationController(subclass) => (0x07, subclass.minor(), subclass.programming_interface()),
			GenericSystemPeripheral(subclass) => (0x08, subclass.minor(), subclass.programming_interface()),
			InputDeviceController(subclass) => (0x09, subclass.minor(), subclass.programming_interface()),
			DockingStation(subclass) => (0x0A, subclass.minor(), subclass.programming_interface()),
			Processor(subclass) => (0x0B, subclass.minor(), subclass.programming_interface()),
			SerialBusController(subclass) => (0x0C, subclass.minor(), subclass.programming_interface()),
			WirelessController(subclass) => (0x0D, subclass.minor(), subclass.programming_interface()),
			IntelligentInputOutputController(subclass) => (0x0E, subclass.minor(), subclass.programming_interface()),
			SatelliteCommunicationsController(subclass) => (0x0F, subclass.minor(), subclass.programming_interface()),
			EncryptionController(subclass) => (0x10, subclass.minor(), subclass.programming_interface()),
			SignalProcessingController(subclass) => (0x11, subclass.minor(), subclass.programming_interface()),
			ProcessingAccelerators(subclass) => (0x12, subclass.minor(), subclass.programming_interface()),
			NonEssentialInstrumentation(subclass) => (0x13, subclass.minor(), subclass.programming_interface()),
			Unassigned(subclass) => (0xFF, subclass.minor(), subclass.programming_interface()),
		}
	}
}
