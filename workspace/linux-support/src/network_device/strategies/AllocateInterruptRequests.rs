// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct AllocateInterruptRequests<'a>
{
	bus_info_name: PciDeviceAddress,
	
	network_interface_name: &'a NetworkInterfaceName,
	
	device_name: &'a [u8],
	
	interrupt_request_affinities: &'a mut InterruptRequestAffinities,
	
	msi_x_interrupt_request_naming_strategy: &'a Box<dyn MsiXInterruptRequestNamingStrategy>,
}

impl<'a> AllocateInterruptRequests<'a>
{
	#[inline(always)]
	fn new(bus_info_name: PciDeviceAddress, network_interface_name: &'a NetworkInterfaceName, device_name: &'a [u8], interrupt_request_affinities: &'a mut InterruptRequestAffinities, msi_x_interrupt_request_naming_strategy: &'a Box<dyn MsiXInterruptRequestNamingStrategy>) -> Self
	{
		Self
		{
			bus_info_name,
			network_interface_name,
			device_name,
			interrupt_request_affinities,
			msi_x_interrupt_request_naming_strategy,
		}
	}
	
	fn allocate_interrupt_requests_for_controllers_and_control_queues(&mut self, administrative_queue_hyper_thread: HyperThread)
	{
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.controller(self.bus_info_name, self.network_interface_name, self.device_name)
		{
			self.add_interrupt_request_affinity(interrupt_name, administrative_queue_hyper_thread)
		}
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.control_queue(self.bus_info_name, self.network_interface_name, self.device_name)
		{
			self.add_interrupt_request_affinity(interrupt_name, administrative_queue_hyper_thread)
		}
	}
	
	fn allocate_interrupt_requests_for_paired_receive_transmit_queue_identifier(&mut self, paired_receive_transmit_queue: QueueIdentifier, paired_receive_transmit_queue_hyper_thread: HyperThread)
	{
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.paired_receive_transmit_queue(self.bus_info_name, self.network_interface_name, self.device_name, paired_receive_transmit_queue)
		{
			self.add_interrupt_request_affinity(interrupt_name, paired_receive_transmit_queue_hyper_thread)
		}
		
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.receive_queue(self.bus_info_name, self.network_interface_name, self.device_name, paired_receive_transmit_queue)
		{
			self.add_interrupt_request_affinity(interrupt_name, paired_receive_transmit_queue_hyper_thread)
		}
		
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.transmit_queue(self.bus_info_name, self.network_interface_name, self.device_name, paired_receive_transmit_queue)
		{
			self.add_interrupt_request_affinity(interrupt_name, paired_receive_transmit_queue_hyper_thread)
		}
	}
	
	fn add_all_queues_fallback(&mut self, associated_hyper_threads_for_paired_receive_transmit_queue_pairs: &HyperThreads)
	{
		if let Some(interrupt_name) = self.msi_x_interrupt_request_naming_strategy.all_queues_fallback(self.bus_info_name, self.network_interface_name, self.device_name)
		{
			for hyper_thread in associated_hyper_threads_for_paired_receive_transmit_queue_pairs.iterate()
			{
				self.add_interrupt_request_affinity(interrupt_name.clone(), hyper_thread)
			}
		}
	}
	
	#[inline(always)]
	fn add_interrupt_request_affinity(&mut self, action: InterruptRequestActionName, hyper_thread: HyperThread)
	{
		self.interrupt_request_affinities.add_interrupt_request_affinity(action, hyper_thread)
	}
}
