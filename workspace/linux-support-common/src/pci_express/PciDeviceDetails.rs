// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI device details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub struct PciDeviceDetails
{
	pub vendor_and_device: PciVendorAndDevice,
	pub subsystem_vendor_and_subsystem_device: PciVendorAndDevice,
	pub class: Option<PciDeviceClass>,
	pub revision: u8,
	pub associated_numa_node: Option<NumaNode>,
	pub associated_hyper_threads: BTreeSet<HyperThread>,
	pub permitted_hyper_threads: u32,
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
