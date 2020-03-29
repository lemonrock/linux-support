// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI device details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub struct PciDeviceDetails
{
	/// Typically this is the 'Chipset Manufacturer'.
	pub vendor_and_device: PciVendorAndDevice,

	/// Typically this is the 'Card Manufacturer'.
	pub subsystem_vendor_and_subsystem_device: PciVendorAndDevice,

	pub class: Option<PciDeviceClass>,

	pub revision: u8,

	pub associated_numa_node: Option<NumaNode>,

	pub associated_hyper_threads_bit_set: BitSet<HyperThread>,

	pub associated_hyper_threads_bitmask: BitSet<HyperThread>,

	pub d3cold_allowed: Option<bool>,

	pub interrupt_request_line: u8, // This should have a type and a range of permitted values.

	/// PCI Express only.
	pub current_link_speed_and_width: Option<LinkSpeedAndWidth>,

	/// PCI Express only.
	pub maximum_link_speed_and_width: Option<LinkSpeedAndWidth>,

	pub enabled: bool,

	pub msi_and_msi_x_enabled: bool,

	pub driver: Option<PciDriverName>,
}

impl PciDeviceDetails
{
	/// Is this an ethernet device?
	#[inline(always)]
	pub fn is_ethernet(&self) -> bool
	{
		self.class == Some(PciDeviceClass::NetworkController(NetworkController::Ethernet))
	}
}
