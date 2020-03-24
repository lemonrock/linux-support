// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::pci_express::PciDevicePhysicalOrVirtualFunction;
use crate::pci_express::classification::PciDeviceIdentifier;
use crate::pci_express::definitions::ethernet::intel::Intel200SeriesAnd300SeriesMediaAccessControlBlockType;


/// Maps a PCI device identifier for an Intel Ethernet PCI device, if supported.
#[inline(always)]
pub fn map(pci_device_identifier: PciDeviceIdentifier) -> Option<(Intel200SeriesAnd300SeriesMediaAccessControlBlockType, PciDevicePhysicalOrVirtualFunction)>
{
	use self::Intel200SeriesAnd300SeriesMediaAccessControlBlockType::*;
	use self::PciDevicePhysicalOrVirtualFunction::*;

	match pci_device_identifier.into()
	{
		_82575EBCopper | _82575EBFibreSerDes | _82575GBCopperQuad => Some((_82575, PhysicalFunction)),
		_82576_ | _82576Fibre | _82576SerDes | _82576CopperQuad | _82576ET2CopperQuad | _82576NS | _82576NSSerDes | _82576SerDesQuad => Some((_82576, PhysicalFunction)),

		_82576VirtualFunction | _82576VirtualFunctionHV => Some((MostVirtualFunction, VirtualFunction)),

		_82580Copper | _82580Fibre | _82580SerDes | _82580SerialGigabitMediaIndependentInterface | _82580CopperDual | _82580FibreQuad | DH89xxCCSerialGigabitMediaIndependentInterface | DH89xxCCSerDes | DH89xxCCBackplane | DH89xxCCSmallFormFactorPluggable => Some((_82580, PhysicalFunction)),

		I210Copper | I210OEM1Copper | I210ITCopper | I210Fibre | I210SerDes | I210SerialGigabitMediaIndependentInterface | I210CopperFlashless | I210SerDesFlashless | I210SerialGigabitMediaIndependentInterfaceFlashless => Some((I210, PhysicalFunction)),

		I211Copper => Some((I211, PhysicalFunction)),

		I350Copper | I350Fibre | I350SerDes | I350SerialGigabitMediaIndependentInterface | I350DA4 => Some((I350, PhysicalFunction)),

		I350VirtualFunction_ | I350VirtualFunctionHV => Some((I350VirtualFunction, VirtualFunction)),

		I354Backplane1Gigabit | I354SerialGigabitMediaIndependentInterface | I354Backplane2Dot5Gigabit => Some((I354, PhysicalFunction)),

		_ => None,
	}
}

const _82575EBCopper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10A7);
const _82575EBFibreSerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10A9);
const _82575GBCopperQuad: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10D6);

const _82576_: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10C9);
const _82576Fibre: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10E6);
const _82576SerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10E7);
const _82576CopperQuad: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10E8);
const _82576ET2CopperQuad: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1526);
const _82576NS: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x150A);
const _82576NSSerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1518);
const _82576SerDesQuad: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x150D);

const _82576VirtualFunction: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x10CA);
const _82576VirtualFunctionHV: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x152D);

const _82580Copper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x150E);
const _82580Fibre: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x150F);
const _82580SerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1510);
const _82580SerialGigabitMediaIndependentInterface: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1511);
const _82580CopperDual: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1516);
const _82580FibreQuad: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1527);
const DH89xxCCSerialGigabitMediaIndependentInterface: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x0438);
const DH89xxCCSerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x043A);
const DH89xxCCBackplane: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x043C);
const DH89xxCCSmallFormFactorPluggable: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x0440);

const I210Copper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1533);
const I210OEM1Copper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1534);
const I210ITCopper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1535);
const I210Fibre: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1536);
const I210SerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1537);
const I210SerialGigabitMediaIndependentInterface: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1538);
const I210CopperFlashless: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x157B);
const I210SerDesFlashless: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x157C);
const I210SerialGigabitMediaIndependentInterfaceFlashless: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x15F6);

const I211Copper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1539);

const I350Copper: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1521);
const I350Fibre: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1522);
const I350SerDes: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1523);
const I350SerialGigabitMediaIndependentInterface: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1524);
const I350DA4: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1546);

const I350VirtualFunction_: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1520);
const I350VirtualFunctionHV: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x152F);

const I354Backplane1Gigabit: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1F40);
const I354SerialGigabitMediaIndependentInterface: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1F41);
const I354Backplane2Dot5Gigabit: PciDeviceIdentifier = PciDeviceIdentifier::new_unchecked(0x1F45);
