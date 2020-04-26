// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A PCI device class.
///
/// See <https://pcisig.com/sites/default/files/files/PCI_Code-ID_r_1_10__v8_Nov_2017.pdf>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
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

impl FromBytes for Either<PciDeviceClass, (u8, u8, u8)>
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let u24 = u32::parse_hexadecimal_number_lower_case_with_0x_prefix_fixed_width(bytes, 6)?;
		Ok(PciDeviceClass::parse(u24))
	}
}

impl PciDeviceClass
{
	/// Returns a recognised, valid combination or 'major, minor, programming_interface'.
	#[inline(always)]
	pub(crate) fn parse(u24: u32) -> Either<Self, (u8, u8, u8)>
	{
		#[inline(always)]
		const fn extract_u8(u24: u32, byte_index: u32) -> u8
		{
			let shift = 8 * byte_index;
			((u24 & (0xFF << shift)) >> shift) as u8
		}

		macro_rules! parse
		{
			($type: ident, $class: ident, $subclass: ident, $programming_interface: ident) =>
			{
				{
					if let Some(subclass) = $type::parse($subclass, $programming_interface)
					{
						Either::Left(PciDeviceClass::$type(subclass))
					}
					else
					{
						Either::Right(($class, $subclass, $programming_interface))
					}
				}
			}
		}

		let class = extract_u8(u24, 2);
		let subclass = extract_u8(u24, 1);
		let programming_interface = extract_u8(u24, 0);
		match class
		{
			0x00 => parse!(Legacy, class, subclass, programming_interface),
			0x01 => parse!(MassStorageController, class, subclass, programming_interface),
			0x02 => parse!(NetworkController, class, subclass, programming_interface),
			0x03 => parse!(DisplayController, class, subclass, programming_interface),
			0x04 => parse!(MultimediaController, class, subclass, programming_interface),
			0x05 => parse!(MemoryController, class, subclass, programming_interface),
			0x06 => parse!(Bridge, class, subclass, programming_interface),
			0x07 => parse!(CommunicationController, class, subclass, programming_interface),
			0x08 => parse!(GenericSystemPeripheral, class, subclass, programming_interface),
			0x09 => parse!(InputDeviceController, class, subclass, programming_interface),
			0x0A => parse!(DockingStation, class, subclass, programming_interface),
			0x0B => parse!(Processor, class, subclass, programming_interface),
			0x0C => parse!(SerialBusController, class, subclass, programming_interface),
			0x0D => parse!(WirelessController, class, subclass, programming_interface),
			0x0E => parse!(IntelligentInputOutputController, class, subclass, programming_interface),
			0x0F => parse!(SatelliteCommunicationsController, class, subclass, programming_interface),
			0x10 => parse!(EncryptionController, class, subclass, programming_interface),
			0x11 => parse!(SignalProcessingController, class, subclass, programming_interface),
			0x12 => parse!(ProcessingAccelerators, class, subclass, programming_interface),
			0x13 => parse!(NonEssentialInstrumentation, class, subclass, programming_interface),
			0xFF => parse!(Unassigned, class, subclass, programming_interface),

			_ => Either::Right((class, subclass, programming_interface)),
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
