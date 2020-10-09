// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AmazonEnaMsiXInterruptRequestNamingStrategy;

impl MsiXInterruptRequestNamingStrategy for AmazonEnaMsiXInterruptRequestNamingStrategy
{
	#[inline(always)]
	fn controller(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		None
	}
	
	#[inline(always)]
	fn control_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		let string: String = bus_info_name.into();
		Some(into_action_name(format!("ena-mgmnt@pci:{:?}", string)))
	}
	
	#[inline(always)]
	fn paired_receive_transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], paired_receive_transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let network_interface_name: &str = network_interface_name.into();
		let paired_receive_transmit_queue: i16 = paired_receive_transmit_queue.into();
		Some(into_action_name(format!("{}-Tx-Rx-{}", network_interface_name, paired_receive_transmit_queue)))
	}
	
	#[inline(always)]
	fn receive_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], receive_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		None
	}
	
	#[inline(always)]
	fn transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		None
	}
	
	#[inline(always)]
	fn all_queues_fallback(&self, _bus_info_name: PciDeviceAddress, _network_interface_name: &NetworkInterfaceName, _device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		None
	}
}
