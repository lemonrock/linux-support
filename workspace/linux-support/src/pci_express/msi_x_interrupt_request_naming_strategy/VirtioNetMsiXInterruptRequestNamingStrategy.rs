// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VirtioNetMsiXInterruptRequestNamingStrategy;

impl MsiXInterruptRequestNamingStrategy for VirtioNetMsiXInterruptRequestNamingStrategy
{
	/// `device_name` is something like `virtio0`.
	#[inline(always)]
	fn controller(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		let device_name: &str = unsafe { from_utf8_unchecked(device_name) };
		Some(into_action_name(format!("{}-config", device_name)))
	}
	
	/// `device_name` is something like `virtio0`.
	///
	/// Not always present.
	#[inline(always)]
	fn control_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8]) -> Option<InterruptRequestActionName>
	{
		let device_name: &str = unsafe { from_utf8_unchecked(device_name) };
		Some(into_action_name(format!("{}-control", device_name)))
	}
	
	#[inline(always)]
	fn paired_receive_transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], paired_receive_transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		None
	}
	
	/// `device_name` is something like `virtio0`.
	///
	/// May not be present.
	#[inline(always)]
	fn receive_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], receive_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let device_name: &str = unsafe { from_utf8_unchecked(device_name) };
		let receive_queue: i16 = receive_queue.into();
		Some(into_action_name(format!("{}-input.{}", device_name, receive_queue)))
	}
	
	/// `device_name` is something like `virtio0`.
	///
	/// May not be present.
	#[inline(always)]
	fn transmit_queue(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let device_name: &str = unsafe { from_utf8_unchecked(device_name) };
		let transmit_queue: i16 = transmit_queue.into();
		Some(into_action_name(format!("{}-output.{}", device_name, transmit_queue)))
	}
	
	/// `device_name` is something like `virtio0`.
	///
	/// May not be present.
	#[inline(always)]
	fn all_queues_fallback(&self, bus_info_name: PciDeviceAddress, network_interface_name: &NetworkInterfaceName, device_name: &[u8], transmit_queue: QueueIdentifier) -> Option<InterruptRequestActionName>
	{
		let device_name: &str = unsafe { from_utf8_unchecked(device_name) };
		Some(into_action_name(format!("{}-virtqueues", device_name)))
	}
}
