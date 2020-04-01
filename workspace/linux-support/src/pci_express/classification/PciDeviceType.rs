// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a PCI device type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PciDeviceType
{
	class: PciDeviceClass,
	vendor_and_device: PciVendorAndDevice,

	/// If `None`, then use (PciVendorIdentifier::AnyOrInvalidRaw, PciDeviceClass::Unassigned(Unassigned::Unassigned))
	subsystem_vendor_and_device: Option<PciVendorAndDevice>,
}

impl PciDeviceType
{
	/// New instance.
	#[inline(always)]
	pub fn new(class: PciDeviceClass, vendor_and_device: PciVendorAndDevice, subsystem_vendor_and_device: Option<PciVendorAndDevice>) -> Self
	{
		Self
		{
			class,
			vendor_and_device,
			subsystem_vendor_and_device,
		}
	}
}
