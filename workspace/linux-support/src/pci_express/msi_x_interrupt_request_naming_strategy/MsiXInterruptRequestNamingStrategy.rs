// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait MsiXInterruptRequestNamingStrategy: Sync + Debug
{
	fn controller(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>;
	
	fn control_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>;
	
	fn paired_receive_transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], paired_receive_transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>;
	
	fn receive_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], receive_queue: QueueIdentifier) -> Option<InterruptRequestActionName>;
	
	fn transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>;
	
	fn all_queues_fallback(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>;
}
