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
	
	/// Forward Error Correction (FEC).
	#[serde(default)] pub forward_error_correction: Option<ForwardErrorCorrectionCode>,
	
	/// Pause configuration.
	#[serde(default)] pub pause_configuration: Option<PauseConfiguration>,
	
	/// Energy Efficient Ethernet (EEE).
	#[serde(default)] pub energy_efficient_ethernet: Option<EnergyEfficientEthernetConfiguration>,
	
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

with coalsecing, need to be able to set individual parameters.

with setting channels, the RSS table is reset on mlx cards.

--set-rxfh-indir
--config-nfc / --config-ntuple
--per-queue ... eg coalesce
--set-priv-flags

rss / rx hash indirection

const char rss_hash_func_strings[ETH_RSS_HASH_FUNCS_COUNT][ETH_GSTRING_LEN] = {
	[ETH_RSS_HASH_TOP_BIT] =	"toeplitz",
	[ETH_RSS_HASH_XOR_BIT] =	"xor",
	[ETH_RSS_HASH_CRC32_BIT] =	"crc32",
};

Indirection RSS hash table

This table can be configured. For example, to make sure all traffic goes only to CPUs #0-#5 (the first NUMA node in our setup), we run:

client$ sudo ethtool -X eth2 weight 1 1 1 1 1 1 0 0 0 0 0
server$ sudo ethtool -X eth3 weight 1 1 1 1 1 1 0 0 0 0 0




Finally, ensure the interrupts of multiqueue network cards are evenly distributed between CPUs. The irqbalance service is stopped and the interrupts are manually assigned. For simplicity let's pin the RX queue #0 to CPU #0, RX queue #1 to CPU #1 and so on.

https://docs.gz.ro/tuning-network-cards-on-linux.html
https://blog.cloudflare.com/how-to-achieve-low-latency/
https://serverfault.com/questions/772380/how-to-tell-if-nic-has-multiqueue-enabled

client$ (let CPU=0; cd /sys/class/net/eth2/device/msi_irqs/;
         for IRQ in *; do
            echo $CPU > /proc/irq/$IRQ/smp_affinity_list
            let CPU+=1
         done)
server$ (let CPU=0; cd /sys/class/net/eth3/device/msi_irqs/;
         for IRQ in *; do
            echo $CPU > /proc/irq/$IRQ/smp_affinity_list
            let CPU+=1
         done)



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
		
		if let Some(forward_error_correction) = self.forward_error_correction
		{
			validate(network_device_input_output_control.set_forward_error_correction(forward_error_correction), CouldNotSetForwardErrorConnection)?
		}
		
		if let Some(pause_configuration) = self.pause_configuration
		{
			validate(network_device_input_output_control.set_pause(pause_configuration), CouldNotSetPause)?
		}
		
		if let Some(ref energy_efficient_ethernet) = self.energy_efficient_ethernet
		{
			validate(network_device_input_output_control.set_energy_efficient_ethernet(energy_efficient_ethernet), CouldNotSetForwardErrorConnection)?
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
