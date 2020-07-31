// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkDeviceConfiguration
{
	/// Network interface name.
	pub network_interface_name: NetworkInterfaceName,

	/// Driver message level.
	///
	/// Use `NETIF_MSG::empty()` to try to disable all messages.
	#[serde(default)] pub driver_message_level: Option<NETIF_MSG>,
	
	/// Disable Wake-on-LAN (WoL).
	#[serde(default)] pub disable_wake_on_lan: bool,
	
	/// Feature groups.
	#[serde(default)] pub feature_group_choices: Vec<FeatureGroupChoice>,
	
	/// Tunables.
	#[serde(default)] pub tunables: Vec<Box<dyn Tunable>>,
	
	/// Change coalesce configuration.
	#[serde(default)] pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// Maximize the number of channels?
	#[serde(default)] pub maximize_number_of_channels: bool,

	/// Maximize pending queue depths?
	#[serde(default)] pub maximize_pending_queue_depths: bool,
}



xxx;
/*

per queue coalesce

rss / rx hash indirection

const char rss_hash_func_strings[ETH_RSS_HASH_FUNCS_COUNT][ETH_GSTRING_LEN] = {
	[ETH_RSS_HASH_TOP_BIT] =	"toeplitz",
	[ETH_RSS_HASH_XOR_BIT] =	"xor",
	[ETH_RSS_HASH_CRC32_BIT] =	"crc32",
};
 */



impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self) -> Result<(Channels, PendingQueueDepths), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		#[inline(always)]
		fn validate<A, E>(change_result: impl FnOnce(&NetworkDeviceInputOutputControl) -> Result<Option<A>, E>, error: impl FnOnce(E) -> GlobalNetworkDeviceConfigurationError) -> Result<A, GlobalNetworkDeviceConfigurationError>
		{
			change(network_device_input_output_control).map_err(error)?.ok_or(NetworkDeviceDoesNotExist(network_device_input_output_control.network_interface_name()))
		}
		
		let network_device_input_output_control = NetworkDeviceInputOutputControl::new(Cow::Borrowed(&self.network_interface_name))?;
		
		if let Some(driver_message_level) = self.driver_message_level
		{
			validate(network_device_input_output_control.set_driver_message_level(driver_message_level), CouldNotSetDriverMessageLevel)?
		}
		
		if self.disable_wake_on_lan
		{
			validate(network_device_input_output_control.disable_wake_on_lan(), CouldNotDisableWakeOnLan)?
		}
		
		validate(network_device_input_output_control.set_features(FeatureGroupChoice::iter(&self.feature_group_choices)), CouldNotChangeFeatures)?;
		
		for tunable in self.tunables.iter()
		{
			validate(network_device_input_output_control.set_tunable(table), CouldNotChangeTunable)?
		}
		
		if let Some(ref coalesce_configuration) = self.coalesce_configuration
		{
			validate(network_device_input_output_control.change_coalesce_configuration(coalesce_configuration), CouldNotChangeCoalesceConfiguration)?
		}
		
		let channels = validate(network_device_input_output_control.maximize_number_of_channels(self.maximize_number_of_channels), CouldNotMaximizeChannels)?;
		
		let pending_queue_depths = validate(network_device_input_output_control.maximize_receive_ring_queues_and_transmit_ring_queue_depths(self.maximize_pending_queue_depths), CouldNotMaximizePendingQueueDepths)?;
		
		Ok((channels, pending_queue_depths))
	}
}
