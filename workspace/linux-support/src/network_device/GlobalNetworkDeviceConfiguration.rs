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
	#[serde(default)] pub driver_message_level: Option<NETIF_MSG>,
	
	/// Link flags to enable and disable.
	///
	/// This is the way to enable promiscuity for a network device (`Some(SettableLinkFlags::Promiscuos, SettableLinkFlags::empty())`) or to disable the use of ARP.
	#[serde(default)] pub link_flags_to_enable_and_disable: Option<(SettableLinkFlags, SettableLinkFlags)>,
	
	/// Transmission queue length, unrelated apparently to `PendingQueueDepths.transmit_pending_queue_depth`.
	///
	/// Default on Linux is 1000 for Ethernet devices.
	/// A value of 128 has been suggested for some improvement in reducing bufferbloat.
	#[serde(default)] pub transmission_queue_length: Option<u32>,
	
	/// Maximum Transmission Unit (MTU).
	///
	/// Does not normally need to change.
	#[serde(default)] pub maximum_transmission_unit: Option<MaximumTransmissionUnitPayloadSize>,
	
	/// Forward Error Correction (FEC).
	#[serde(default)] pub forward_error_correction: Option<ForwardErrorCorrectionCode>,
	
	/// Pause configuration.
	///
	/// Usually only works for physical (non-virtualized) hardware.
	///
	/// Recommended to be turned off for receive and transmit by Intel.
	#[serde(default)] pub pause_configuration: Option<PauseConfiguration>,
	
	/// Energy Efficient Ethernet (EEE).
	///
	/// Not supported by many cards, eg Intel i40e.
	#[serde(default)] pub energy_efficient_ethernet: Option<EnergyEfficientEthernetConfiguration>,
	
	/// Disable Wake-on-LAN.
	///
	/// Usually only works for physical (non-virtualized) hardware.
	#[serde(default)] pub disable_wake_on_lan: bool,
	
	/// Feature groups.
	#[serde(default)] pub feature_group_choices: Vec<FeatureGroupChoice>,
	
	/// Driver-specific private flags.
	#[serde(default)] pub driver_specific_flags_to_change: Option<HashMap<ObjectName32, bool>>,
	
	/// Tunables.
	#[serde(default)] pub tunables: Vec<TunableChoice>,
	
	/// Change coalesce configuration.
	#[serde(default)] pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// Maximize the number of channels?
	///
	/// Changing the number of channels is not possible if an eXpress Data Path (XDP) program is attached.
	/// Changing the number of channels is not possible if Intel Flow Director (fdir) rules are active.
	/// Changing the number of channels is not possible for Intel i40e if traffic classes (TCs) are configured through using the Multiqueue Priority Qdisc (Offloaded Hardware QOS), [MQPRIO](https://www.man7.org/linux/man-pages/man8/tc-mqprio.8.html).
	#[serde(default)] pub maximize_number_of_channels: bool,

	/// Maximize pending queue depths?
	///
	/// Sometimes, *reducing* these to 128 or 256 can help Intel Data I/O Direct (DDIO).
	#[serde(default)] pub maximize_pending_queue_depths: bool,
	
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
	/// Passing `ConfiguredHashSettings` with all fields as `None` effectively does a result to defaults, if the network device supports that, or, for an older device, if supported, reseting the indirection (RETA) table to defaults.
	///
	/// This is always configured after changes to the number of channels, as changes to channels resets RSS hash configuration on some cards (eg Mellanox).
	#[serde(default)] pub receive_side_scaling_hash_configuration: Option<ConfiguredHashSettings>,
	
	/// Change generic receive offload (GRO) flush timeout?
	///
	/// Default is usually `0`.
	#[serde(default)] pub generic_receive_offload_flush_timeout_in_nanoseconds: Option<u32>,
}

impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, network_interface_name: &NetworkInterfaceName) -> Result<(Channels, PendingQueueDepths), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
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
		
		if let Some(ref driver_specific_flags_to_change) = self.driver_specific_flags_to_change
		{
			let all_string_sets = validate(&network_device_input_output_control, network_device_input_output_control.all_string_sets(), CouldNotGetAllStringSets)?;
			validate(&network_device_input_output_control, network_device_input_output_control.set_private_flags(&all_string_sets, driver_specific_flags_to_change), CouldNotChangeDriverSpecificFlags)?;
		}
		
		for tunable_choice in self.tunables.iter()
		{
			validate(&network_device_input_output_control, tunable_choice.set(&network_device_input_output_control), CouldNotChangeTunable)?
		}
		
		if let Some(ref coalesce_configuration) = self.coalesce_configuration
		{
			validate(&network_device_input_output_control, network_device_input_output_control.change_coalesce_configuration(coalesce_configuration), CouldNotChangeCoalesceConfiguration)?
		}
		
		let channels = validate(&network_device_input_output_control, network_device_input_output_control.maximize_number_of_channels(self.maximize_number_of_channels), CouldNotMaximizeChannels)?;
		
		let pending_queue_depths = validate(&network_device_input_output_control, network_device_input_output_control.maximize_receive_ring_queues_and_transmit_ring_queue_depths(self.maximize_pending_queue_depths), CouldNotMaximizePendingQueueDepths)?;
		
		if let Some(ref receive_side_scaling_hash_configuration) = self.receive_side_scaling_hash_configuration
		{
			validate(&network_device_input_output_control, network_device_input_output_control.set_configured_hash_settings(None, receive_side_scaling_hash_configuration), CouldNotConfigureReceiveSideScalingHashConfiguration)?;
		}
		
		if let Some(generic_receive_offload_flush_timeout_in_nanoseconds) = self.generic_receive_offload_flush_timeout_in_nanoseconds
		{
			network_interface_name.set_generic_receive_offload_flush_timeout_in_nanoseconds(sys_path, generic_receive_offload_flush_timeout_in_nanoseconds).map_err(CouldNotSetGenericReceiveOffloadTimeout)?;
		}
		
		Ok((channels, pending_queue_depths))
	}
}
