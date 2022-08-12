// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A profile for a PCI ethernet driver.
#[derive(Debug)]
pub struct DriverProfile
{
	feature_group_choices: Vec<FeatureGroupChoice>,
	
	driver_specific_flags_to_change: HashMap<ObjectName32, bool>,

	tunables: Vec<TunableChoice>,
	
	supports_getting_and_setting_queue_depths: bool,
	
	supports_getting_and_setting_channels: bool,
	
	supported_hash_function_names: HashSet<HashFunctionName>,
	
	best_possible_hash_function_fields_configuration: IndexSet<HashFunctionFieldsConfiguration>,
	
	all_supported_hash_function_fields_configuration: IndexSet<HashFunctionFieldsConfiguration>,

	msi_x_interrupt_request_naming_strategy: Box<dyn MsiXInterruptRequestNamingStrategy>,

	coalescing_strategy: Box<dyn CoalescingStrategy>,
}

impl DriverProfile
{
	/// Configure all multiqueue PCI ethernet devices.
	pub fn configure_all_multiqueue_pci_ethernet_devices(sys_path: &SysPath, proc_path: &ProcPath, device_preferences: &DevicePreferences) -> Result<HashMap<NetworkInterfaceName, (HyperThread, HyperThreads)>, DriverProfileError>
	{
		let all_pci_buses = PciBusAddress::all(sys_path).map_err(FailedToRetrieveAllPciBuses)?;
		
		let mut receive_flow_steering_flow_count = 0;
		let mut configurations = Vec::new();
		let mut interrupt_request_affinities = InterruptRequestAffinities::default();
		let mut hyper_threads = HashMap::new();
		
		let linux_kernel_version = LinuxKernelVersion::parse(proc_path).map_err(FailedToRetrieveLinuxKernelVersion)?;
		for network_interface_name in NetworkInterfaceName::all().map_err(FailedToObtainAllNetworkInterfaceNames)?
		{
			// Loopback interfaces and those without a device do not have this; additionally, loop back interfaces do not have ethtool driver names, etc.
			if let Some(device_name_guess) = network_interface_name.device_name_guess(sys_path)
			{
				let network_device_input_output_control_driver_profile = NetworkDeviceInputOutputControlDriverProfile::new(&network_interface_name)?;
				if let Some((driver_name, driver_version, bus_info_name)) = network_device_input_output_control_driver_profile.driver_name_and_driver_version_and_pci_device_device_address()?
				{
					if let Some(pci_device) = network_interface_name.pci_device(sys_path).map_err(|error| CouldNotGetPciDevice { network_interface_name: network_interface_name.clone(), error })?.ok_or(NoSuchNetworkInterface { network_interface_name: network_interface_name.clone() })?
					{
						// We do not error here as `bus_info_name` could have been abused by a driver we do not want to use.
						if pci_device.address() != bus_info_name
						{
							continue
						}
						
						if let Some(pci_vendor_and_device) = pci_device.vendor_and_device()
						{
							if let Some(driver_profile) = DriverProfileChoice::find_driver_profile(&linux_kernel_version, &driver_name, &driver_version, pci_vendor_and_device)
							{
								let (configuration, receive_flow_steering_flow_count_per_device, administrative_queue_hyper_thread, associated_hyper_threads_for_paired_receive_transmit_queue_pairs) = driver_profile.to_global_network_device_configuration(&network_device_input_output_control_driver_profile, &network_interface_name, &pci_device, &all_pci_buses, &device_name_guess, device_preferences, &mut interrupt_request_affinities)?;
								receive_flow_steering_flow_count += receive_flow_steering_flow_count_per_device;
								configurations.push((network_interface_name.clone(), configuration));
								hyper_threads.insert(network_interface_name, (administrative_queue_hyper_thread, associated_hyper_threads_for_paired_receive_transmit_queue_pairs));
							}
						}
					}
				}
			}
		}
		
		Self::configure(sys_path, proc_path, receive_flow_steering_flow_count, configurations, interrupt_request_affinities)?;
		
		Ok(hyper_threads)
	}
	
	// NOTE: Order of configuration operations is significant.
	fn configure(sys_path: &SysPath, proc_path: &ProcPath, receive_flow_steering_flow_count: usize, configurations: Vec<(NetworkInterfaceName, GlobalNetworkDeviceConfiguration)>, interrupt_request_affinities: InterruptRequestAffinities) -> Result<(), ConfigureDriverProfileError>
	{
		ReceiveFlowSteeringFlowCount::capped(receive_flow_steering_flow_count).set_global_maximum(proc_path).map_err(CouldNotConfigureReceiveFlowSteeringCount)?;
		
		for (network_interface_name, configuration) in configurations
		{
			configuration.configure(sys_path, &network_interface_name).map_err(|error| CouldNotDoGlobalNetworkDeviceConfiguration { network_interface_name: network_interface_name.clone(), error })?;
		}
		
		interrupt_request_affinities.set_hyper_thread_affinities(sys_path, proc_path).map_err(CouldNotConfigureInterruptRequestAffinities)?;
		
		Ok(())
	}
	
	fn to_global_network_device_configuration(&self, network_device_input_output_control_driver_profile: &NetworkDeviceInputOutputControlDriverProfile, network_interface_name: &NetworkInterfaceName, pci_device: &PciDevice, all_pci_buses: &HashMap<PciBusAddress, io::Result<PciBusDetails>>, device_name_guess: &[u8], device_preferences: &DevicePreferences, interrupt_request_affinities: &mut InterruptRequestAffinities) -> Result<(GlobalNetworkDeviceConfiguration, usize, HyperThread, HyperThreads), DriverProfileError>
	{
		let (paired_receive_transmit_queue_count, number_of_channels, receive_queues, transmit_queues, receive_flow_steering_flow_count, administrative_queue_hyper_thread, associated_hyper_threads_for_paired_receive_transmit_queue_pairs) = self.queue_configuration(network_device_input_output_control_driver_profile, network_interface_name, pci_device, all_pci_buses, device_name_guess, device_preferences, interrupt_request_affinities)?;
		
		let receive_side_scaling_hash_function_configuration = self.receive_side_scaling_hash_function_configuration(network_device_input_output_control_driver_profile, paired_receive_transmit_queue_count)?;
		
		let receive_side_scaling_hash_function_fields_configuration = self.receive_side_scaling_hash_function_fields_configuration();
		
		Ok
		(
			(
				GlobalNetworkDeviceConfiguration
				{
					transmission_queue_length: device_preferences.transmission_queue_length,
					
					generic_receive_offload_flush_timeout_in_nanoseconds: Some(device_preferences.generic_receive_offload_flush_timeout_in_nanoseconds),
					
					counter_to_decrement_before_processing_hard_interrupt_requests: Some(device_preferences.counter_to_decrement_before_processing_hard_interrupt_requests),
					
					feature_group_choices: self.feature_group_choices.clone(),
					
					driver_specific_flags_to_change: self.driver_specific_flags_to_change.clone(),
					
					tunables: self.tunables.clone(),
					
					coalesce_configuration: Some(self.coalesce_configuration(device_preferences)),
					
					number_of_channels,
					
					pending_queue_depths: if self.supports_getting_and_setting_queue_depths
					{
						use self::SetToSpecificValueOrMaximize::*;
						
						Some
						(
							match &device_preferences.queue_depths
							{
								&Maximize => Maximize,
								
								&SpecificValue((receive_queue_depth, transmit_queue_depth)) => SpecificValue(PendingQueueDepths
								{
									receive_pending_queue_depth: Some(receive_queue_depth),
									receive_mini_pending_queue_depth: None,
									receive_jumbo_pending_queue_depth: None,
									transmit_pending_queue_depth: Some(transmit_queue_depth)
								})
							}
						)
					}
					else
					{
						None
					},
					
					receive_queues,
					
					transmit_queues,
					
					receive_side_scaling_hash_function_configuration,
					
					receive_side_scaling_hash_function_fields_configuration,
					
					driver_message_level: None,
					
					link_flags_to_enable_and_disable: None,
					
					maximum_transmission_unit: None,
					
					forward_error_correction: None,
					
					pause_configuration: None,
					
					energy_efficient_ethernet: None,
					
					disable_wake_on_lan: false,
					
					per_queue_coalesce_configuration: HashMap::new(),
				},
				
				receive_flow_steering_flow_count,
				
				administrative_queue_hyper_thread,
				
				associated_hyper_threads_for_paired_receive_transmit_queue_pairs,
			)
		)
	}
	
	fn queue_configuration(&self, network_device_input_output_control_driver_profile: &NetworkDeviceInputOutputControlDriverProfile, network_interface_name: &NetworkInterfaceName, pci_device: &PciDevice, all_pci_buses: &HashMap<PciBusAddress, io::Result<PciBusDetails>>, device_name_guess: &[u8], device_preferences: &DevicePreferences, interrupt_request_affinities: &mut InterruptRequestAffinities) -> Result<(QueueCount, Option<SetToSpecificValueOrMaximize<Channels>>, HashMap<QueueIdentifier, GlobalNetworkDeviceReceiveQueueConfiguration>, HashMap<QueueIdentifier, GlobalNetworkDeviceTransmitQueueConfiguration>, usize, HyperThread, HyperThreads), DriverProfileError>
	{
		let (administrative_queue_hyper_thread, (associated_hyper_threads_for_paired_receive_transmit_queue_pairs, associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count)) = Self::administrative_queue_hyper_thread_and_associated_hyper_threads_for_paired_receive_transmit_queue_pairs_and_maximum_receive_transmit_queue_count(network_interface_name, pci_device, all_pci_buses)?;
		
		let (paired_receive_transmit_queue_count, number_of_channels) = if self.supports_getting_and_setting_channels
		{
			let paired_receive_transmit_queue_count = network_device_input_output_control_driver_profile.paired_receive_transmit_queue_count(associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count, NetworkDeviceInputOutputControlDriverProfile::maximum_paired_receive_transmit_queue_count)?;
			(paired_receive_transmit_queue_count, Some(Self::channels(paired_receive_transmit_queue_count)))
		}
		else
		{
			let paired_receive_transmit_queue_count = network_device_input_output_control_driver_profile.paired_receive_transmit_queue_count(associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count, NetworkDeviceInputOutputControlDriverProfile::current_number_of_receive_queue_count)?;
			(paired_receive_transmit_queue_count, None)
		};
		
		let mut receive_queues = paired_receive_transmit_queue_count.new_queue_configurations();
		let mut transmit_queues = paired_receive_transmit_queue_count.new_queue_configurations();
		
		let mut allocate_interrupt_requests = AllocateInterruptRequests::new(pci_device.address(), network_interface_name, device_name_guess, interrupt_request_affinities, &self.msi_x_interrupt_request_naming_strategy);
		allocate_interrupt_requests.allocate_interrupt_requests_for_controllers_and_control_queues(administrative_queue_hyper_thread);
		
		let mut receive_flow_steering_flow_count = 0;
		debug_assert_eq!(associated_hyper_threads_for_paired_receive_transmit_queue_pairs.len(), associated_hyper_threads_for_paired_receive_transmit_queue_pairs_count.into());
		for (queue_identifier, queue_hyper_thread) in paired_receive_transmit_queue_count.queue_identifiers().zip(associated_hyper_threads_for_paired_receive_transmit_queue_pairs.iterate())
		{
			allocate_interrupt_requests.allocate_interrupt_requests_for_paired_receive_transmit_queue_identifier(queue_identifier, queue_hyper_thread);
			
			let receive_flow_steering_table_count_per_queue = device_preferences.receive_flow_steering_table_count_per_queue;
			receive_queues.insert(queue_identifier, GlobalNetworkDeviceReceiveQueueConfiguration::use_receive_side_scaling_if_possible(Some(queue_hyper_thread), receive_flow_steering_table_count_per_queue));
			receive_flow_steering_flow_count += receive_flow_steering_table_count_per_queue;
			
			transmit_queues.insert(queue_identifier, GlobalNetworkDeviceTransmitQueueConfiguration::linux_default_with_one_to_one_receive_to_transmit_packet_steering(queue_identifier));
		}
		
		allocate_interrupt_requests.add_all_queues_fallback(&associated_hyper_threads_for_paired_receive_transmit_queue_pairs);
		
		Ok((paired_receive_transmit_queue_count, number_of_channels, receive_queues, transmit_queues, receive_flow_steering_flow_count, administrative_queue_hyper_thread, associated_hyper_threads_for_paired_receive_transmit_queue_pairs))
	}
	
	#[inline(always)]
	fn administrative_queue_hyper_thread_and_associated_hyper_threads_for_paired_receive_transmit_queue_pairs_and_maximum_receive_transmit_queue_count(network_interface_name: &NetworkInterfaceName, pci_device: &PciDevice, all_pci_buses: &HashMap<PciBusAddress, io::Result<PciBusDetails>>) -> Result<(HyperThread, (HyperThreads, QueueCount)), DriverProfileError>
	{
		let mut associated_hyper_threads = pci_device.validated_associated_hyper_threads(all_pci_buses);
		let actual_number = associated_hyper_threads.len();
		if actual_number < 2
		{
			Err
			(
				AtLeastTwoHyperThreadsAreRequired
				{
					network_interface_name: network_interface_name.clone(),
					
					actual_number,
				}
			)
		}
		else
		{
			let administrative_queue_hyper_thread = associated_hyper_threads.pop_first();
			
			Ok((administrative_queue_hyper_thread, (QueueCount::from_number_of_hyper_threads_capped_to_inclusive_maximum(associated_hyper_threads))))
		}
	}
	
	#[inline(always)]
	const fn channels(paired_receive_transmit_queue_count: QueueCount) -> SetToSpecificValueOrMaximize<Channels>
	{
		SetToSpecificValueOrMaximize::SpecificValue(Channels::new_combined_only(paired_receive_transmit_queue_count))
	}
	
	#[inline(always)]
	fn receive_side_scaling_hash_function_configuration(&self, network_device_input_output_control_driver_profile: &NetworkDeviceInputOutputControlDriverProfile, paired_receive_transmit_queue_count: QueueCount) -> Result<Option<HashFunctionConfiguration>, DriverProfileError>
	{
		lazy_static!
		{
			static ref DesiredHashFunctionNamesInPreferenceOrder: IndexSet<HashFunctionName> = indexset!
			[
				HashFunctionName::Toeplitz,
				HashFunctionName::CyclicRedundancyCheck32,
				HashFunctionName::ExclusiveOr,
			];
		}
		
		for desired_hash_function_name in DesiredHashFunctionNamesInPreferenceOrder.iter()
		{
			if self.supported_hash_function_names.contains(desired_hash_function_name)
			{
				return Ok(Some(Self::hash_function_configuration(network_device_input_output_control_driver_profile, paired_receive_transmit_queue_count, *desired_hash_function_name, &HashFunctionSeed::toeplitz_symmetric())?))
			}
		}
		Ok(None)
	}
	
	#[inline(always)]
	fn hash_function_configuration(network_device_input_output_control_driver_profile: &NetworkDeviceInputOutputControlDriverProfile, paired_receive_transmit_queue_count: QueueCount, hash_function_name: HashFunctionName, hash_function_seed: &HashFunctionSeed) -> Result<HashFunctionConfiguration, DriverProfileError>
	{
		let existing_hash_function_configuration = network_device_input_output_control_driver_profile.existing_hash_function_configuration()?;
		
		let indirection_table_length_u32 = existing_hash_function_configuration.indirection_table_length_u32()?.ok_or(DoesNotSupportAHashFunctionIndirectionTable)?;
		let seed = existing_hash_function_configuration.new_seed_matching_in_length(hash_function_seed).ok_or(DoesNotSupportAHashFunctionSeed)?;
		
		Ok
		(
			HashFunctionConfiguration
			{
				function: Some(hash_function_name),
				
				indirection_table: Some(IndirectionTable::calculate_indirection_table(paired_receive_transmit_queue_count, indirection_table_length_u32, FairWeightQueueStrategy)),
				
				seed: Some(seed),
			}
		)
	}
	
	#[inline(always)]
	fn receive_side_scaling_hash_function_fields_configuration(&self) -> IndexSet<HashFunctionFieldsConfiguration>
	{
		let receive_side_scaling_hash_function_fields_configuration = self.best_possible_hash_function_fields_configuration.clone();
		if cfg!(debug_assertions)
		{
			for combination in self.best_possible_hash_function_fields_configuration.iter()
			{
				debug_assert!(self.all_supported_hash_function_fields_configuration.contains(combination));
			}
		}
		receive_side_scaling_hash_function_fields_configuration
	}
	
	#[inline(always)]
	fn coalesce_configuration(&self, device_preferences: &DevicePreferences) -> CoalesceConfiguration
	{
		self.coalescing_strategy.coalesce_configuration(true, &device_preferences.receive_coalescing_preference, &device_preferences.transmit_coalescing_preference)
	}
	
	/// See <https://www.kernel.org/doc/html/latest/networking/device_drivers/ethernet/amazon/ena.html>.
	#[inline(always)]
	fn amazon_ena() -> Self
	{
		Self
		{
			feature_group_choices: vec!
			[
				// Scatter-Gather (subset of `ethtool_sg`).
				ethtool_sg - NETIF_F_FRAGLIST_BIT,
				
				// Transmission Control Protocol Segment Offload (TSO) (subset of `ethtool_tso`).
				ethtool_tso - NETIF_F_TSO_MANGLEID_BIT,
				
				FeatureGroupChoice::enable_one(NETIF_F_HIGHDMA_BIT),
				
				ethtool_rx,
				
				// If the NETIF_F_RXHASH flag is set, the 32-bit result of the hash function delivered in the Rx CQ descriptor is set in the received SKB.
				ethtool_rxhash,
				
				// Internet Protocol, TCP and UDP checksums for receive and transmit.
				internet_protocols_checksum,
			],
			
			driver_specific_flags_to_change: HashMap::new(),
			
			// The driver-allocated SKB for frames received from Rx handling using NAPI context.
			// The allocation method depends on the size of the packet.
			// If the frame length is larger than rx_copybreak, napi_get_frags() is used, otherwise netdev_alloc_skb_ip_align() is used, the buffer content is copied (by CPU) to the SKB, and the buffer is recycled.
			tunables: vec!
			[
				TunableChoice::ReceiveCopyBreakPoint(ReceiveCopyBreakPointTunable(CopyBreakPointTunable(256))),
			],
			
			supports_getting_and_setting_queue_depths: true,
			
			supports_getting_and_setting_channels: true,
			
			supported_hash_function_names: fast_secure_hash_set!
			[
				HashFunctionName::Toeplitz,
				HashFunctionName::CyclicRedundancyCheck32,
			],
			
			best_possible_hash_function_fields_configuration: HashFunctionFieldsConfiguration::amazon_ena_best_possible(),
			
			all_supported_hash_function_fields_configuration: HashFunctionFieldsConfiguration::amazon_ena_valid_combinations_of_hash_function_fields_configuration(),
			
			msi_x_interrupt_request_naming_strategy: Box::new(AmazonEnaMsiXInterruptRequestNamingStrategy),
			
			coalescing_strategy: Box::new(AmazonEnaCoalescingStrategy),
		}
	}
	
	/// A Linux kernel forked derivative of version `4.1.0-k`.
	fn intel_ixgbevf_linux_fork() -> Self
	{
		Self::intel_ixgbevf
		(
			false,
			false,
			IndexSet::new(),
			IndexSet::new(),
		)
	}
	
	/// Out of tree; supplied by Intel.
	fn intel_ixgbevf_intel_fork_x540_or_earlier() -> Self
	{
		Self::intel_ixgbevf
		(
			false,
			true,
			IndexSet::new(),
			IndexSet::new(),
		)
	}
	
	/// Out of tree; supplied by Intel.
	fn intel_ixgbevf_intel_fork_x550_or_later() -> Self
	{
		Self::intel_ixgbevf
		(
			true,
			true,
			HashFunctionFieldsConfiguration::intel_ixgbevf_intel_fork_for_x550_or_later_valid_combinations_of_hash_function_fields_configuration_best_possible(),
			HashFunctionFieldsConfiguration::intel_ixgbevf_intel_fork_for_x550_or_later_valid_combinations_of_hash_function_fields_configuration(),
		)
	}
	
	#[inline(always)]
	fn virtio_net() -> Self
	{
		Self
		{
			feature_group_choices: vec!
			[
				// Scatter-Gather (subset of `ethtool_sg`).
				ethtool_sg - NETIF_F_FRAGLIST_BIT,
				
				// Transmission Control Protocol Segment Offload (TSO) (subset of `ethtool_tso`).
				ethtool_tso - NETIF_F_TSO_ECN_BIT,
				
				FeatureGroupChoice::enable_one(NETIF_F_HIGHDMA_BIT),
				
				ethtool_rx,
				
				// Depends on underlying physical hardware.
				FeatureGroupChoice::enable_one(NETIF_F_HW_VLAN_CTAG_FILTER_BIT),
				
				// Depends on underlying physical hardware.
				FeatureGroupChoice::enable_one(NETIF_F_GSO_ROBUST_BIT),
				
				// Depends on underlying physical hardware.
				FeatureGroupChoice::enable_one(NETIF_F_LRO_BIT),
			],
			
			driver_specific_flags_to_change: fast_secure_hash_map!
			[
			],
			
			// The driver-allocated SKB for frames received from Rx handling using NAPI context.
			// The allocation method depends on the size of the packet.
			// If the frame length is larger than rx_copybreak, napi_get_frags() is used, otherwise netdev_alloc_skb_ip_align() is used, the buffer content is copied (by CPU) to the SKB, and the buffer is recycled.
			tunables: vec!
			[
			],
			
			supports_getting_and_setting_queue_depths: false,
			
			supports_getting_and_setting_channels: true,
			
			supported_hash_function_names: fast_secure_hash_set!
			[
			],
			
			best_possible_hash_function_fields_configuration: IndexSet::new(),
			
			all_supported_hash_function_fields_configuration: IndexSet::new(),
			
			msi_x_interrupt_request_naming_strategy: Box::new(VirtioNetMsiXInterruptRequestNamingStrategy),
			
			coalescing_strategy: Box::new(VirtioNetCoalescingStrategy),
		}
	}
	
	#[inline(always)]
	fn intel_ixgbevf(has_ethtool_rxhash: bool, has_NETIF_F_GRO_BIT: bool, best_possible_hash_function_fields_configuration: IndexSet<HashFunctionFieldsConfiguration>, all_supported_hash_function_fields_configuration: IndexSet<HashFunctionFieldsConfiguration>) -> Self
	{
		let IXGBEVF_PRIV_FLAGS_LEGACY_RX = ObjectName32::try_from("legacy-rx").unwrap();
		
		let mut miscellaneous = fast_secure_hash_set!
		[
			NETIF_F_GSO_PARTIAL_BIT,
			NETIF_F_HW_VLAN_CTAG_FILTER_BIT,
			NETIF_F_HW_VLAN_CTAG_RX_BIT,
			NETIF_F_HW_VLAN_CTAG_TX_BIT,
			NETIF_F_SCTP_CRC_BIT
		];
		
		if has_NETIF_F_GRO_BIT
		{
			miscellaneous.insert(NETIF_F_GRO_BIT);
		}
		
		if has_ethtool_rxhash
		{
			miscellaneous.insert(NETIF_F_RXHASH_BIT);
		}
		
		Self
		{
			feature_group_choices: vec!
			[
				// Scatter-Gather (subset of `ethtool_sg`).
				ethtool_sg - NETIF_F_FRAGLIST_BIT,
				
				// Transmission Control Protocol Segment Offload (TSO) (subset of `ethtool_tso`).
				ethtool_tso - NETIF_F_TSO_ECN_BIT,
				
				FeatureGroupChoice::enable_one(NETIF_F_HIGHDMA_BIT),
				
				ethtool_rx,
				
				internet_protocols_checksum_in_hardware,
				
				generic_send_offload_encapsulation,
				
				FeatureGroupChoice::enable(miscellaneous),
			],
			
			driver_specific_flags_to_change: fast_secure_hash_map!
			[
				IXGBEVF_PRIV_FLAGS_LEGACY_RX => false,
			],
			
			tunables: vec!
			[
			],
			
			supports_getting_and_setting_queue_depths: true,
			
			supports_getting_and_setting_channels: false,
			
			supported_hash_function_names: fast_secure_hash_set!
			[
			],
			
			best_possible_hash_function_fields_configuration,
			
			all_supported_hash_function_fields_configuration,
			
			msi_x_interrupt_request_naming_strategy: Box::new(IntelIxgbevfMsiXInterruptRequestNamingStrategy),
			
			coalescing_strategy: Box::new(IntelIxgbevfCoalescingStrategy),
		}
	}
}
