// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Network device diagnostic from `ioctl()`.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceInputOutputControlDiagnostic
{
	/// Driver message level (bitflags).
	pub driver_message_level: NETIF_MSG,
	
	/// Driver name.
	pub driver_name: ObjectName32,
	
	/// If `Some` will not be empty.
	pub driver_version: Option<ObjectName32>,
	
	/// If `Some` will not be empty.
	///
	/// May not be a PCI address.
	pub device_bus_device_address: Option<BusDeviceAddress>,
	
	/// `None` if not supported.
	pub link_is_up: Option<bool>,
	
	/// `None` if not supported.
	pub link_settings: Option<LinkSettings>,
	
	/// Device features.
	pub device_features: DeviceFeatures,
	
	/// Private flags.
	pub private_flags: HashSet<ObjectName32>,
	
	/// All string sets
	pub all_string_sets: AllStringSets,
	
	/// `None` if not supported.
	pub current_queue_depths_and_maximum_queue_depths: Option<(PendingQueueDepths, PendingQueueDepths)>,
	
	/// `None` if not supported.
	pub current_number_of_channels_and_maximum_number_of_channels: Option<(Channels, Channels)>,
	
	/// Receive ring queue count.
	///
	/// If only one implicit ring is supported, this is returned as `1`.
	///
	/// Ordinarily should match `self.current_number_of_channels_and_maximum_number_of_channels.1.combined_count`.
	pub receive_ring_queue_count: QueueCount,
	
	pub default_context_receive_side_scaling_flow_hash_key_configurations: HashMap<ReceiveSideScalingFlowHashKeyName, Option<ReceiveSideScalingFlowHashKeyConfiguration>>,
	
	/// Configured receive-side scaling (RSS) hash settings for the default context (`None`).
	///
	/// There doesn't seem to be a simple way to list other contexts.
	pub default_context_configured_receive_side_scaling_hash_settings: ConfiguredHashSettings,
	
	/// `None` if not supported.
	pub wake_on_lan: Option<WakeOnLanInformation>,
	
	/// `None` if not supported.
	pub energy_efficient_ethernet: Option<EnergyEfficientEthernetInformation>,
	
	/// `None` if not supported.
	pub pause: Option<PauseConfiguration>,
	
	/// `None` if not supported.
	pub forward_error_correction: Option<HashSet<ForwardErrorCorrectionCode>>,
	
	/// `None` if not supported.
	pub downshift_retries_count: Option<DownshiftRetriesCountTunable>,
	
	/// `None` if not supported.
	pub energy_detect_power_down_milliseconds: Option<EnergyDetectPowerDownMillisecondsTunable>,
	
	/// `None` if not supported.
	pub receive_copy_break: Option<ReceiveCopyBreakTunable>,
	
	/// `None` if not supported.
	pub transmit_copy_break: Option<TransmitCopyBreakTunable>,
	
	/// `None` if not supported.
	pub fast_link_down_milliseconds: Option<FastLinkDownMillisecondsTunable>,
	
	/// `None` if not supported.
	pub priority_flow_control_storm_prevention: Option<PriorityFlowControlStormPreventionTunable>,
	
	/// `None` if not supported.
	pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// `None` if not supported.
	pub per_queue_coalsece_configurations: Option<HashMap<QueueIdentifier, CoalesceConfiguration>>,
	
	/// `None` if not supported.
	pub timestamping: Option<NetworkDeviceTimestampingInformation>,
	
	/// `None` if not supported.
	pub firmware: Option<NetworkDeviceFirmware>,
	
	/// `None` if not supported.
	pub expansion_eeprom: Option<ExpansionEepromBinaryData>,
	
	/// `None` if not supported.
	///
	/// Can include diagnostic monitoring information, so may differ from dump to dump.
	pub plugin_module_eeprom: Option<PluginModuleEepromBinaryData>,
	
	/// `None` if not supported, `Some(empty)` if supported but no statistics are provided.
	pub nic_statistics: Option<HashMap<ObjectName32, u64>>,
	
	/// `None` if not supported, `Some(empty)` if supported but no statistics are provided.
	pub phy_statistics: Option<HashMap<ObjectName32, u64>>,
	
	/// `None` if not supported.
	pub registers: Option<NetworkDeviceRegisters>,
}

impl NetworkDeviceInputOutputControlDiagnostic
{
	fn gather(network_interface_name: &NetworkInterfaceName) -> DiagnosticUnobtainableResult<Self>
	{
		let network_device_input_output_control = NetworkDeviceInputOutputControl::new(Cow::Borrowed(network_interface_name)).map_err(DiagnosticUnobtainable::from)?;
		
		macro_rules! exists
		{
			($result: expr) =>
			{
				match $result.map_err(DiagnosticUnobtainable::from)?
				{
					None => return Err(DiagnosticUnobtainable(format!("Network interface name no longer seems to be available"))),
					Some(value) => value
				}
			}
		}
		
		let driver_and_device_information = exists!(network_device_input_output_control.driver_and_device_information());
		let all_string_sets = exists!(network_device_input_output_control.all_string_sets());
		Ok
		(
			Self
			{
				driver_message_level: exists!(network_device_input_output_control.driver_message_level()),
				
				link_is_up: exists!(network_device_input_output_control.link_is_up()),
				
				link_settings: exists!(network_device_input_output_control.link_settings()),
				
				device_features: exists!(network_device_input_output_control.features()),
				
				private_flags: exists!(network_device_input_output_control.private_flags(&all_string_sets)),
				
				current_number_of_channels_and_maximum_number_of_channels: exists!(network_device_input_output_control.number_of_channels()),
				
				current_queue_depths_and_maximum_queue_depths: exists!(network_device_input_output_control.receive_ring_queues_and_transmit_ring_queue_depths()),
				
				wake_on_lan: exists!(network_device_input_output_control.wake_on_lan()),
				
				energy_efficient_ethernet: exists!(network_device_input_output_control.energy_efficient_ethernet()),
				
				pause: exists!(network_device_input_output_control.pause()),
				
				forward_error_correction: exists!(network_device_input_output_control.forward_error_correction()),
				
				default_context_configured_receive_side_scaling_hash_settings: exists!(network_device_input_output_control.configured_receive_side_scaling_hash_settings(None)),
				
				receive_ring_queue_count: exists!(network_device_input_output_control.receive_ring_queue_count()),
				
				default_context_receive_side_scaling_flow_hash_key_configurations:
				{
					let mut configurations = HashMap::with_capacity(ReceiveSideScalingFlowHashKeyName::COUNT);
					for key_name in ReceiveSideScalingFlowHashKeyName::iter()
					{
						configurations.insert(key_name, exists!(network_device_input_output_control.receive_side_scaling_flow_hash_key(key_name, None)))?;
					}
					configurations
				},
				
				downshift_retries_count: exists!(network_device_input_output_control.tunable()),
				
				energy_detect_power_down_milliseconds: exists!(network_device_input_output_control.tunable()),
				
				receive_copy_break: exists!(network_device_input_output_control.tunable()),
				
				transmit_copy_break: exists!(network_device_input_output_control.tunable()),
				
				fast_link_down_milliseconds: exists!(network_device_input_output_control.tunable()),
				
				priority_flow_control_storm_prevention: exists!(network_device_input_output_control.tunable()),
				
				coalesce_configuration: exists!(network_device_input_output_control.coalesce_configuration()),
				
				per_queue_coalsece_configurations: exists!(network_device_input_output_control.per_queue_coalesce_configuration(None)),
				
				timestamping: exists!(network_device_input_output_control.timestamping()),
				
				firmware: exists!(network_device_input_output_control.firmware(&driver_and_device_information)),
				
				expansion_eeprom: exists!(network_device_input_output_control.expansion_eeprom(&driver_and_device_information)),
				
				plugin_module_eeprom: exists!(network_device_input_output_control.plugin_module_eeprom()),
				
				nic_statistics: exists!(network_device_input_output_control.nic_statistics(&all_string_sets)),
				
				phy_statistics: exists!(network_device_input_output_control.phy_statistics(&all_string_sets)),
				
				registers: exists!(network_device_input_output_control.registers(&driver_and_device_information)),
				
				all_string_sets,
				
				driver_name: driver_and_device_information.driver_name,
				
				driver_version: driver_and_device_information.driver_version,
				
				device_bus_device_address: driver_and_device_information.device_bus_device_address,
			}
		)
	}
}
