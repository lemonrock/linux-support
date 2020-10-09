// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkDeviceConfiguration
{
	/// Driver message level.
	///
	/// Use `NETIF_MSG::empty()` to try to disable all messages.
	///
	/// * Supported by Amazon ENA.
	/// * Supported by Intel ixgbevf.
	#[serde(default)] pub driver_message_level: Option<NETIF_MSG>,
	
	/// Link flags to enable and disable.
	///
	/// This is the way to enable promiscuity for a network device (`Some(SettableLinkFlags::Promiscuous, SettableLinkFlags::empty())`) or to disable the use of ARP.
	///
	/// Support is independent of driver (eg Amazon ENA).
	#[serde(default)] pub link_flags_to_enable_and_disable: Option<(SettableLinkFlags, SettableLinkFlags)>,
	
	/// Transmission queue length, unrelated apparently to `PendingQueueDepths.transmit_pending_queue_depth`.
	///
	/// Whilst there is one setting, each transmission queue is affected.
	///
	/// Default on Linux is 1000 for Ethernet devices.
	/// A value of 128 has been suggested for some improvement in reducing bufferbloat.
	///
	/// Support is independent of driver (eg Amazon ENA) and depends on configuration Traffic Class; currently only the `pfifo_fast` traffic class uses this value.
	#[serde(default)] pub transmission_queue_length: Option<u32>,
	
	/// Maximum Transmission Unit (MTU).
	///
	/// * Amazon ENA supports an inclusive minimum of `ENA_MIN_MTU` (128).
	/// 	* With XDP, it has a maximum of `ENA_XDP_MAX_MTU`, which is `ENA_PAGE_SIZE (4096) - ETH_HLEN (14) - ETH_FCS_LEN (4) - VLAN_HLEN (4) - XDP_PACKET_HEADROOM (256) - SKB_DATA_ALIGN(sizeof(struct skb_shared_info))`.
	/// * Intel ixgbevf supports an inclusive minimum of `ETH_MIN_MTU` (68) and an inclusive maximum of either `ETH_DATA_LEN (1500) + ETH_FCS_LEN (4)` (82599 VF) or `IXGBE_MAX_JUMBO_FRAME_SIZE (9728) - ETH_HLEN (14) - ETH_FCS_LEN (4)`.
	#[serde(default)] pub maximum_transmission_unit: Option<MaximumTransmissionUnitPayloadSize>,
	
	/// Forward Error Correction (FEC).
	///
	/// * Not supported by Amazon ENA.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub forward_error_correction: Option<ForwardErrorCorrectionCode>,
	
	/// Pause configuration.
	///
	/// Usually only works for physical (non-virtualized) hardware.
	///
	/// Recommended to be turned off for receive and transmit by Intel.
	///
	/// * Not supported by Amazon ENA.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub pause_configuration: Option<PauseConfiguration>,
	
	/// Energy Efficient Ethernet (EEE).
	///
	/// Not supported by many cards, eg Intel i40e.
	///
	/// * Not supported by Amazon ENA.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub energy_efficient_ethernet: Option<EnergyEfficientEthernetConfiguration>,
	
	/// Disable Wake-on-LAN.
	///
	/// Usually only works for physical (non-virtualized) hardware.
	///
	/// * Not supported by Amazon ENA.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub disable_wake_on_lan: bool,
	
	/// Feature groups.
	///
	/// * Amazon ENA supports:-
	/// 	* NETIF_F_HIGHDMA
	/// 	* NETIF_F_IPV6_CSUM
	/// 	* NETIF_F_IP_CSUM
	/// 	* NETIF_F_RXCSUM
	/// 	* NETIF_F_RXHASH
	/// 	* NETIF_F_SG
	/// 	* NETIF_F_TSO
	/// 	* NETIF_F_TSO6
	/// 	* NETIF_F_TSO_ECN
	/// * Intel ixgbevf (Linux) supports:-
	/// 	* NETIF_F_GSO_GRE
	/// 	* NETIF_F_GSO_GRE_CSUM
	/// 	* NETIF_F_GSO_IPXIP4
	/// 	* NETIF_F_GSO_IPXIP6
	/// 	* NETIF_F_GSO_PARTIAL
	/// 	* NETIF_F_GSO_UDP_TUNNEL
	/// 	* NETIF_F_GSO_UDP_TUNNEL_CSUM
	/// 	* NETIF_F_HIGHDMA
	/// 	* NETIF_F_HW_CSUM
	/// 	* NETIF_F_HW_VLAN_CTAG_FILTER
	/// 	* NETIF_F_HW_VLAN_CTAG_RX
	/// 	* NETIF_F_HW_VLAN_CTAG_TX
	/// 	* NETIF_F_RXCSUM
	/// 	* ?NETIF_F_RXHASH
	/// 	* NETIF_F_SCTP_CRC
	/// 	* NETIF_F_SG
	/// 	* NETIF_F_TSO
	/// 	* NETIF_F_TSO6
	/// 	* NETIF_F_TSO_MANGLEID
	/// * Intel ixgbevf (Intel) supports:-
	/// 	* NETIF_F_GRO
	/// 	* NETIF_F_GSO_GRE
	/// 	* NETIF_F_GSO_GRE_CSUM
	/// 	* NETIF_F_GSO_IPXIP4
	/// 	* NETIF_F_GSO_IPXIP6
	/// 	* NETIF_F_GSO_PARTIAL
	/// 	* NETIF_F_GSO_UDP_TUNNEL
	/// 	* NETIF_F_GSO_UDP_TUNNEL_CSUM
	/// 	* NETIF_F_HIGHDMA
	/// 	* NETIF_F_HW_CSUM
	/// 	* NETIF_F_HW_VLAN_CTAG_FILTER
	/// 	* NETIF_F_HW_VLAN_CTAG_RX
	/// 	* NETIF_F_HW_VLAN_CTAG_TX
	/// 	* NETIF_F_RXCSUM
	/// 	* ?NETIF_F_RXHASH
	/// 	* NETIF_F_SCTP_CRC
	/// 	* NETIF_F_SG
	/// 	* NETIF_F_TSO
	/// 	* NETIF_F_TSO6
	/// 	* NETIF_F_TSO_MANGLEID
	#[serde(default)] pub feature_group_choices: Vec<FeatureGroupChoice>,
	
	/// Driver-specific private flags.
	///
	/// * Not supported by Amazon ENA.
	/// * Supported by Intel ixgbevf for `IXGBEVF_PRIV_FLAGS_LEGACY_RX` ("legacy-rx").
	#[serde(default)] pub driver_specific_flags_to_change: HashMap<ObjectName32, bool>,
	
	/// Tunables.
	///
	/// * Supported by Amazon ENA for `ReceiveCopyBreakTunable`.
	/// 	* Value must be less than current Maximum Transmission Unit (MTU).
	/// 	* Default is `ENA_DEFAULT_RX_COPYBREAK`	`(256 - NET_IP_ALIGN)` which is 256 for x86_64.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub tunables: Vec<TunableChoice>,
	
	/// Maximize the number of channels?
	///
	/// Changing the number of channels is not possible if an eXpress Data Path (XDP) program is attached.
	/// Changing the number of channels is not possible if Intel Flow Director (fdir) rules are active.
	/// Changing the number of channels is not possible for Intel i40e if traffic classes (TCs) are configured through using the Multiqueue Priority Qdisc (Offloaded Hardware QOS), [MQPRIO](https://www.man7.org/linux/man-pages/man8/tc-mqprio.8.html).
	///
	/// * Supported by Amazon ENA for `combined_count` only.
	/// 	* Value must be at least 1 (`ENA_MIN_NUM_IO_QUEUES`).
	/// 	* Value must be `2 * queues <= adapter->max_num_io_queues (NetworkDeviceInputOutputControl.channels().1)`.
	/// 	* Absolute maximum is `ENA_MAX_NUM_IO_QUEUES` (128) but see logic in `ena_calc_max_io_queue_num()` in Linux source `ena_netdev.c`.
	/// 		* Reduced to smaller of number of online CPUs, available tx and rx descriptors and number of MSI-X IRQs less one.
	/// 		* Reduced by 1 for maximum number of MSI-X IRQs.
	/// 	* Default is maximum number of queues.
	/// * Not supported by Intel ixgbevf.
	#[serde(default)] pub number_of_channels: Option<SetToSpecificValueOrMaximize<Channels>>,
	
	/// Maximize pending queue depths?
	///
	/// Sometimes, *reducing* these to 128 or 256 can help Intel Data I/O Direct (DDIO).
	///
	/// * Amazon ENA supports:-
	/// 	* `rx_pending`
	/// 		* default size of `ENA_DEFAULT_RING_SIZE` (1024).
	/// 		* minimum size of `ENA_MIN_RING_SIZE` (256).
	/// 		* must be a power of 2.
	/// 	* `tx_pending`
	/// 		* default size of `ENA_DEFAULT_RING_SIZE` (1024).
	/// 		* minimum size of `ENA_MIN_RING_SIZE` (256).
	/// 		* must be a power of 2.
	/// * Intel ixgbevf supports:-
	/// 	* `rx_pending`
	/// 		* minimum size of `IXGBEVF_MIN_RXD` (64).
	/// 		* maximum size of `IXGBEVF_MAX_RXD` (4096).
	/// 		* must be a multiple of `IXGBE_REQ_RX_DESCRIPTOR_MULTIPLE` (8).
	/// 	* `tx_pending`
	/// 		* minimum size of `IXGBEVF_MIN_TXD` (64).
	/// 		* maximum size of `IXGBEVF_MAX_TXD` (4096).
	/// 		* must be a multiple of `IXGBE_REQ_TX_DESCRIPTOR_MULTIPLE` (8).
	#[serde(default)] pub pending_queue_depths: Option<SetToSpecificValueOrMaximize<PendingQueueDepths>>,
	
	/// Change coalesce configuration.
	///
	/// * Amazon ENA supports `tx_coalesce_usecs`, `rx_coalesce_usecs` and `use_adaptive_rx_coalesce` if 'ena_com_interrupt_moderation_supported'.
	/// * Intel ixgbevf supports `rx_coalesce_usecs` and, ordinarily, `tx_coalesce_usecs`.
	/// 	* Values must be between `1` (inclusive) and `0x00000FF8 (IXGBE_MAX_EITR) >> 2` exclusive.
	#[serde(default)] pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// Change per-queue coalesce configuration.
	///
	/// * Unsupported by Amazon ENA.
	/// * Unsupported by Intel ixgbevf.
	#[serde(default)] pub per_queue_coalesce_configuration: HashMap<QueueIdentifier, CoalesceConfiguration>,
	
	/// Receive queue settings.
	///
	/// Support is independent of driver (eg Amazon ENA).
	#[serde(default)] pub receive_queues: HashMap<QueueIdentifier, GlobalNetworkDeviceReceiveQueueConfiguration>,
	
	/// Transmit queue settings.
	///
	/// Support is independent of driver (eg Amazon ENA).
	#[serde(default)] pub transmit_queues: HashMap<QueueIdentifier, GlobalNetworkDeviceTransmitQueueConfiguration>,
	
	/// Adjust receive side scaling (RSS) hash configuration:-
	///
	/// * Change the hash function (eg to Toeplitz);
	/// * Change the key to the hash function (normally 40 or 52 bytes);
	/// * Change the indirection (RETA) table (the weighting of hash function results to receive queue indices).
	///
	/// Doing this generically is very hard, as the specifics are very much tied to the network device.
	/// This could be done semi-generically using an algorithm that picks a key size, hash function and RETA table given known card supported values (existing configuration and number of receive queue rings), NUMA settings and CPU counts.
	///
	/// Older network devices may only support changing the indirection (RETA) table.
	/// Passing `HashFunctionConfiguration` with all fields as `None` effectively does a result to defaults, if the network device supports that, or, for an older device, if supported, reseting the indirection (RETA) table to defaults.
	///
	/// This is always configured after changes to the number of channels, as changes to channels resets RSS hash configuration on some cards (eg Mellanox).
	///
	/// * Supported by Amazon ENA:-
	/// 	* `function` can be either Toeplitz or CRC32.
	/// 	* Hash key size is `ENA_HASH_KEY_SIZE` (40).
	/// 	* Indirection table size is `ENA_RX_RSS_TABLE_SIZE` (128).
	/// * Supported by Intel ixgbevf but ***read-only*** ie can not be configured:-
	/// 	* `function` is only ever Toeplitz.
	/// 	* Hash key size is `IXGBEVF_RSS_HASH_KEY_SIZE` (40).
	/// 	* Indirection table size is either `IXGBEVF_82599_RETA_SIZE` (128) or `IXGBEVF_X550_VFRETA_SIZE` (64).
	#[serde(default)] pub receive_side_scaling_hash_function_configuration: Option<HashFunctionConfiguration>,
	
	/// Adjust the fields from an incoming packet used to generate a hash.
	///
	/// Support of various configurations is very spotty.
	///
	/// * Supported by Amazon ENA.
	/// 	* Does not support `discard`.
	/// 	* Does not support anything other than RSS default context (fails if `FLOW_RSS` is set).
	/// 	* For Ethernet
	/// 		* `ETHER_FLOW`.
	/// 			* Source address (not possible to configure with ethtool).
	/// 			* `include_ethernet_destination_address`
	/// 	* For Internet Protocol version 4
	/// 		* IPV4_FLOW
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 		* `TransmissionControlProtocolOverInternetProtocolVersion4`.
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 			* `include_source_port`.
	/// 			* `include_destination_port`.
	/// 		* `UserDatagramProtocolOverInternetProtocolVersion4`.
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 			* `include_source_port`.
	/// 			* `include_destination_port`.
	/// 	* For Internet Protocol version 6
	/// 		* IPV6_FLOW.
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 		* `TransmissionControlProtocolOverInternetProtocolVersion6`.
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 			* `include_source_port`.
	/// 			* `include_destination_port`.
	/// 		* `UserDatagramProtocolOverInternetProtocolVersion6`.
	/// 			* `include_internet_protocol_source_address`.
	/// 			* `include_internet_protocol_destination_address`.
	/// 			* `include_source_port`.
	/// 			* `include_destination_port`.
	/// * Unsupported by Intel ixgbevf.
	#[serde(default)] pub receive_side_scaling_hash_function_fields_configuration: IndexSet<HashFunctionFieldsConfiguration>,
	
	/// Change generic receive offload (GRO) flush timeout?
	///
	/// Default is usually `0`.
	///
	/// NAPI polling tuning parameter.
	/// Support is independent of driver (eg Amazon ENA).
	#[serde(default)] pub generic_receive_offload_flush_timeout_in_nanoseconds: Option<u32>,
	
	/// Counter to decrement before processing hard (real) interrupt requests.
	#[serde(default)] pub counter_to_decrement_before_processing_hard_interrupt_requests: Option<Option<NonZeroU32>>,
}

impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, network_interface_name: &NetworkInterfaceName) -> Result<(), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		use self::SetToSpecificValueOrMaximize::*;
		
		#[inline(always)]
		fn validate<A, E>(network_device_input_output_control: &NetworkDeviceInputOutputControl, change_result: Result<Option<A>, E>, error: impl FnOnce(E) -> GlobalNetworkDeviceConfigurationError) -> Result<A, GlobalNetworkDeviceConfigurationError>
		{
			change_result.map_err(error)?.ok_or(NetworkDeviceDoesNotExist(network_device_input_output_control.network_interface_name()))
		}
		
		let network_device_input_output_control = NetworkDeviceInputOutputControl::new(Cow::Borrowed(network_interface_name))?;
		
		if let Some(driver_message_level) = self.driver_message_level
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_driver_message_level(driver_message_level), CouldNotSetDriverMessageLevel)?;
		}
		
		if let Some((enable, disable)) = self.link_flags_to_enable_and_disable
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_link_flags(enable, disable), CouldNotSetLinkFlags)?;
		}
		
		if let Some(transmission_queue_length) = self.transmission_queue_length
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_transmission_queue_length(transmission_queue_length), CouldNotSetTransmissionQueueLength)?;
		}
		
		if let Some(maximum_transmission_unit) = self.maximum_transmission_unit
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_maximum_transmission_unit(maximum_transmission_unit), CouldNotSetMaximumTransmissionUnit)?;
		}
		
		if let Some(forward_error_correction) = self.forward_error_correction
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_forward_error_correction(forward_error_correction), CouldNotSetForwardErrorConnection)?
		}
		
		if let Some(pause_configuration) = self.pause_configuration
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_pause(pause_configuration), CouldNotSetPause)?
		}
		
		if let Some(ref energy_efficient_ethernet) = self.energy_efficient_ethernet
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_energy_efficient_ethernet(energy_efficient_ethernet), CouldNotSetForwardErrorConnection)?
		}
		
		if self.disable_wake_on_lan
		{
			validate(&network_device_input_output_control, network_device_input_output_control.disable_wake_on_lan(), CouldNotDisableWakeOnLan)?
		}
		
		validate(&network_device_input_output_control, network_device_input_output_control.set_features(FeatureGroupChoice::iter(&self.feature_group_choices)), CouldNotChangeFeatures)?;
		
		if !self.driver_specific_flags_to_change.is_empty()
		{
			let all_string_sets = validate(&network_device_input_output_control, network_device_input_output_control.all_string_sets(), CouldNotGetAllStringSets)?;
			validate(&network_device_input_output_control, network_device_input_output_control.set_private_flags(&all_string_sets, &self.driver_specific_flags_to_change), CouldNotChangeDriverSpecificFlags)?;
		}
		
		for tunable_choice in self.tunables.iter()
		{
			validate(&network_device_input_output_control, tunable_choice.set(&network_device_input_output_control), CouldNotChangeTunable)?
		}
		
		if let Some(ref number_of_channels) = self.number_of_channels
		{
			match number_of_channels
			{
				&SpecificValue(ref number_of_channels) => validate(&network_device_input_output_control, network_device_input_output_control.set_number_of_channels(number_of_channels), CouldNotChangeChannels)?,
				
				&Maximize => validate(&network_device_input_output_control, network_device_input_output_control.maximize_number_of_channels(), CouldNotChangeChannels)?,
			}
		}
		if let Some(ref pending_queue_depths) = self.pending_queue_depths
		{
			match pending_queue_depths
			{
				&SpecificValue(ref pending_queue_depths) => validate(&network_device_input_output_control, network_device_input_output_control.set_receive_ring_queues_and_transmit_ring_queue_depths(pending_queue_depths), CouldNotChangePendingQueueDepths)?,
				
				&Maximize => validate(&network_device_input_output_control, network_device_input_output_control.maximize_receive_ring_queues_and_transmit_ring_queue_depths(), CouldNotChangePendingQueueDepths)?,
			}
		}
		
		if let Some(ref coalesce_configuration) = self.coalesce_configuration
		{
			validate(&network_device_input_output_control, network_device_input_output_control.change_coalesce_configuration(coalesce_configuration), CouldNotChangeCoalesceConfiguration)?
		}
		
		if !self.per_queue_coalesce_configuration.is_empty()
		{
			validate(&network_device_input_output_control, network_device_input_output_control.change_per_queue_coalesce_configuration(&self.per_queue_coalesce_configuration), CouldNotChangePerQueueCoalesceConfiguration)?
		}
		
		for (queue_identifier, receive_queue) in self.receive_queues.iter()
		{
			receive_queue.configure(sys_path, &ReceiveSysfsQueue::new(network_interface_name, *queue_identifier))?;
		}
		
		for (queue_identifier, transmit_queue) in self.transmit_queues.iter()
		{
			transmit_queue.configure(sys_path, &TransmitSysfsQueue::new(network_interface_name, *queue_identifier))?;
		}
		
		if let Some(ref receive_side_scaling_hash_configuration) = self.receive_side_scaling_hash_function_configuration
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_configured_hash_settings(None, receive_side_scaling_hash_configuration), CouldNotConfigureReceiveSideScalingHashConfiguration)?;
		}
		
		for receive_side_scaling_hash_key_configuration in self.receive_side_scaling_hash_function_fields_configuration.iter()
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_receive_side_scaling_flow_hash_key(receive_side_scaling_hash_key_configuration, None), CouldNotConfigureHashFunctionFieldsConfiguration)?;
		}
		
		if let Some(generic_receive_offload_flush_timeout_in_nanoseconds) = self.generic_receive_offload_flush_timeout_in_nanoseconds
		{
			network_interface_name.set_generic_receive_offload_flush_timeout_in_nanoseconds(sys_path, generic_receive_offload_flush_timeout_in_nanoseconds).map_err(CouldNotSetGenericReceiveOffloadTimeout)?;
		}
		
		if let Some(counter_to_decrement_before_processing_hard_interrupt_requests) = self.counter_to_decrement_before_processing_hard_interrupt_requests
		{
			network_interface_name.set_counter_to_decrement_before_processing_hard_interrupt_requests(sys_path, counter_to_decrement_before_processing_hard_interrupt_requests).map_err(CouldNotSetNapHardInterruptRequestsCount)?;
		}
		
		Ok(())
	}
}
