// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI device details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PciDeviceDetails
{
	/// Typically this is the 'Chipset Manufacturer'.
	pub vendor_and_device: PciVendorAndDevice,
	
	/// Not always present.
	pub driver: Option<PciDriverName>,
	
	/// Not always present; not present for virtio.
	pub directly_associated_network_devices: Option<HashSet<NetworkInterfaceName>>,

	/// This is not a PCI concept but a Linux concept.
	pub subsystem_name: Box<[u8]>,
	
	/// Typically this is the 'Card Manufacturer'.
	pub subsystem_vendor_and_subsystem_device: PciVendorAndDevice,
	
	#[allow(missing_docs)]
	pub class: EitherPciDeviceClass,
	
	#[allow(missing_docs)]
	pub revision: Revision,
	
	#[allow(missing_docs)]
	pub associated_numa_node: Option<Option<NumaNode>>,
	
	#[allow(missing_docs)]
	pub associated_hyper_threads_bit_set: Option<HyperThreads>,
	
	#[allow(missing_docs)]
	pub associated_hyper_threads_bitmask: Option<HyperThreads>,
	
	#[allow(missing_docs)]
	pub d3cold_allowed: Option<bool>,
	
	#[allow(missing_docs)]
	pub interrupt_request: InterruptRequest,
	
	/// PCI Express only.
	pub current_link_speed_and_width: Option<LinkSpeedAndWidth>,
	
	/// PCI Express only.
	pub maximum_link_speed_and_width: Option<LinkSpeedAndWidth>,
	
	#[allow(missing_docs)]
	pub enabled: bool,
	
	#[allow(missing_docs)]
	pub msi_and_msi_x_enabled: bool,
	
	#[allow(missing_docs)]
	pub msi_and_msi_x_interrupt_requests: Option<HashMap<InterruptRequest, MsiInterruptMode>>,
	
	#[allow(missing_docs)]
	pub alternative_routing_identifier_interpretation_forwarding_enabled: bool,
	
	#[allow(missing_docs)]
	pub resource_files: Vec<ResourceFile>,
	
	#[allow(missing_docs)]
	pub has_rom: bool,
	
	#[allow(missing_docs)]
	pub has_config: bool,
	
	/// Only for `PCI_CLASS_DISPLAY_VGA`.
	pub boot_vga: bool,

	/// Only for a bridge.
	pub bridge: Option<PciBridgeDeviceDetails>,
}

impl PciDeviceDetails
{
	/// Is this an ethernet device?
	#[inline(always)]
	pub fn is_ethernet(&self) -> bool
	{
		self.class == EitherPciDeviceClass(Left(PciDeviceClass::NetworkController(NetworkController::Ethernet)))
	}
}
