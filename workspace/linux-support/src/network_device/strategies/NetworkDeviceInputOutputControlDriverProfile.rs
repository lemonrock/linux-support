// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct NetworkDeviceInputOutputControlDriverProfile<'a>(NetworkDeviceInputOutputControl<'a>);

impl<'a> Deref for NetworkDeviceInputOutputControlDriverProfile<'a>
{
	type Target = NetworkDeviceInputOutputControl<'a>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'a> DerefMut for NetworkDeviceInputOutputControlDriverProfile<'a>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<'a> NetworkDeviceInputOutputControlDriverProfile<'a>
{
	#[inline(always)]
	fn new(network_interface_name: &'a NetworkInterfaceName) -> Result<Self, FindDriverProfileError>
	{
		match NetworkDeviceInputOutputControl::new(Cow::Borrowed(network_interface_name))
		{
			Err(error) => Err(CouldNotCreateNetworkDeviceInputOutputControl { error, network_interface_name: network_interface_name.clone() }),
			
			Ok(network_device_input_output_control) => Ok(Self(network_device_input_output_control))
		}
	}
	
	#[inline(always)]
	fn driver_name_and_driver_version_and_pci_device_device_address(&self) -> Result<Option<(ObjectName32, ObjectName32, PciDeviceAddress)>, FindDriverProfileError>
	{
		let option = self.driver_and_device_information().map_err(|error| CouldNotGetDriverAndDeviceInformation { network_interface_name: self.network_interface_name(), error })?;
		
		let driver_and_device_information = match option.ok_or(NoSuchNetworkInterface { network_interface_name: self.network_interface_name() })?
		{
			None => return Ok(None),
			
			Some(driver_and_device_information) => driver_and_device_information,
		};
		
		let driver_version = match driver_and_device_information.driver_version
		{
			None => return Ok(None),
			
			Some(driver_version) => driver_version,
		};
		
		let pci_device_address = match driver_and_device_information.device_bus_device_address
		{
			None => return Ok(None),
			
			Some(bus_device_address) => match PciDeviceAddress::try_from(bus_device_address)
			{
				Err(_) => return Ok(None),
				Ok(pci_device_address) => pci_device_address,
			}
		};
		
		Ok(Some((driver_and_device_information.driver_name, driver_version, pci_device_address)))
	}
	
	#[inline(always)]
	fn paired_receive_transmit_queue_count(&self, associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count: QueueCount, possible_queue_count: impl FnOnce(&Self) -> Result<QueueCount, DriverProfileError>) -> Result<QueueCount, DriverProfileError>
	{
		let possible_queue_count = possible_queue_count(self)?;
		Ok(min(associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count, possible_queue_count))
	}
	
	#[inline(always)]
	fn existing_hash_function_configuration(&self) -> Result<HashFunctionConfiguration, DriverProfileError>
	{
		self.parse_result(self.configured_receive_side_scaling_hash_settings(None), |error| CouldNotGetReceiveSideScalingHashFunctionConfiguration { network_interface_name: self.network_interface_name(), error })
	}
	
	#[inline(always)]
	fn maximum_paired_receive_transmit_queue_count(&self) -> Result<QueueCount, DriverProfileError>
	{
		let (_current, maxima) = self.parse_result(self.number_of_channels(), |error| CouldNotGetNumberOfChannels { network_interface_name: self.network_interface_name(), error })?.ok_or(DoesNotSupportNumberOfChannels { network_interface_name: self.network_interface_name() })?;
		maxima.receive_and_transmit_channels_count.ok_or(DoesNotSupportCombinedChannels { network_interface_name: self.network_interface_name() })
	}
	
	#[inline(always)]
	fn current_number_of_receive_queue_count(&self) -> Result<QueueCount, DriverProfileError>
	{
		self.parse_result(self.receive_ring_queue_count(), |error| CouldNotGetNumberOfReceiveRingQueues { network_interface_name: self.network_interface_name(), error })
	}
	
	#[inline(always)]
	fn parse_result<T, E>(&self, result: Result<Option<T>, E>, map_error: impl FnOnce(E) -> DriverProfileError) -> Result<T, DriverProfileError>
	{
		let option = result.map_err(map_error)?;
		option.ok_or(DriverProfileError::from(NoSuchNetworkInterface { network_interface_name: self.network_interface_name() }))
	}
	
	#[inline(always)]
	fn network_interface_name(&self) -> NetworkInterfaceName
	{
		use self::Cow::*;
		match self.network_interface_name
		{
			Borrowed(network_interface_name) => network_interface_name.clone(),
			Owned(ref network_interface_name) => network_interface_name.clone()
		}
	}
}
