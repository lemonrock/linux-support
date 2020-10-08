// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct IntelIxgbevfMsiXInterruptRequestNamingStrategy;

impl MsiXInterruptRequestNamingStrategy for IntelIxgbevfMsiXInterruptRequestNamingStrategy
{
	#[inline(always)]
	fn controller(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		let network_interface_name: &str = network_interface_name.into();
		Some(into_action_name(format!("{}", network_interface_name)))
	}
	
	#[inline(always)]
	fn control_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		None
	}
	
	// Note: `Rx` is capitalized but is lower case for `receive_queue()`!
	// Note: `Tx` is capitalized but is lower case for `transmit_queue()`!
	#[inline(always)]
	fn paired_receive_transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], paired_receive_transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let network_interface_name: &str = network_interface_name.into();
		let paired_receive_transmit_queue: u16 = paired_receive_transmit_queue.into();
		Some(into_action_name(format!("{}-TxRx-{}", network_interface_name, paired_receive_transmit_queue)))
	}
	
	// Note: `rx` is lower case but is capitalized for `paired_receive_transmit_queue()`!
	#[inline(always)]
	fn receive_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], receive_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let network_interface_name: &str = network_interface_name.into();
		let receive_queue: u16 = receive_queue.into();
		Some(into_action_name(format!("{}-rx-{}", network_interface_name, receive_queue)))
	}
	
	// Note: `tx` is lower case but is capitalized for `paired_receive_transmit_queue()`!
	#[inline(always)]
	fn transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let network_interface_name: &str = network_interface_name.into();
		let transmit_queue: u16 = transmit_queue.into();
		Some(into_action_name(format!("{}-tx-{}", network_interface_name, transmit_queue)))
	}
	
	#[inline(always)]
	fn all_queues_fallback(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		None
	}
}
