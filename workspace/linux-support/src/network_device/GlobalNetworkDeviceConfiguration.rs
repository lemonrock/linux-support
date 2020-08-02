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
	///
	/// Usually only works for physical (non-virtualized) hardware.
	#[serde(default)] pub pause_configuration: Option<PauseConfiguration>,
	
	/// Energy Efficient Ethernet (EEE).
	///
	/// Not supported by many cards, eg Intel i40e.
	#[serde(default)] pub energy_efficient_ethernet: Option<EnergyEfficientEthernetConfiguration>,
	
	/// Disable Wake-on-LAN (WoL).
	///
	/// Usually only works for physical (non-virtualized) hardware.
	#[serde(default)] pub disable_wake_on_lan: bool,
	
	/// Feature groups.
	#[serde(default)] pub feature_group_choices: Vec<FeatureGroupChoice>,
	
	/// Driver-specific private flags.
	#[serde(default)] pub driver_specific_flags_to_change: Option<HashMap<ObjectName32, bool>>,
	
	/// Tunables.
	#[serde(default)] pub tunables: Vec<Box<dyn Tunable>>,
	
	/// Change coalesce configuration.
	#[serde(default)] pub coalesce_configuration: Option<CoalesceConfiguration>,
	
	/// Maximize the number of channels?
	#[serde(default)] pub maximize_number_of_channels: bool,

	/// Maximize pending queue depths?
	#[serde(default)] pub maximize_pending_queue_depths: bool,
	
	/// Change generic receive offload (GRO) flush timeout?
	///
	/// Default is usually `0`.
	#[serde(default)] pub generic_receive_offload_flush_timeout_in_nanoseconds: Option<u32>,
}


xxx;
/*


--set-rxfh-indir
	do_srxfh
		do_srxfhindir
	
--config-nfc / --config-ntuple
	do_srxclass
		"rx-flow-hash"
		"flow-type"
			do_srxntuple
		"delete"
--per-queue ... eg coalesce

Rework set ring params, eg Intel i40e returns EINVAL for values outside of the range I40E_MIN_NUM_DESCRIPTORS ..= I40E_MAX_NUM_DESCRIPTORS
	eg Intel i40e does not suppport rx_mini_pending or rx_jumbo_pending

Rework coalesce, for example, many devices might only support a subset, eg Intel i40e:-

	.supported_coalesce_params = ETHTOOL_COALESCE_USECS |
				     ETHTOOL_COALESCE_MAX_FRAMES_IRQ |
				     ETHTOOL_COALESCE_USE_ADAPTIVE |
				     ETHTOOL_COALESCE_RX_USECS_HIGH |
				     ETHTOOL_COALESCE_TX_USECS_HIGH,

with setting channels, the RSS table is reset on mlx cards.
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


//  /sys/class/net/eth2/device/msi_irqs may not exist

Use struct InterruptRequest

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

/*
/sys/class/net/eth0

    queues/
        tx-0/
            rps_cpus
                00000000
                read-write
            rps_flow_cnt
                0
                read-write
        rx-0/
            traffic_class
                couldn't be read
            tx_maxrate
                0
                read-write
            tx_timeout
                0 (without line feed)
            xps_cpus
                couldn't be read
                read-write
            xps_rxqs
                0
                read-write
            byte_queue_limits/
                hold_time
                    1000
                    read-write
                inflight
                    0
                    read-only
                limit
                    0
                    read-write
                limit_max
                    1879048192
                    read-write
                limit_min
                    0
                    read-write
    power/
        autosuspend_delay_ms
            couldn't be read
            read-write
        control
            "auto"
            read-write
        runtime_active_time
        runtime_status
            "unsupported"
        runtime_suspended_time
 */

xxxx;



impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath) -> Result<(Channels, PendingQueueDepths), GlobalNetworkDeviceConfigurationError>
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
		
		if let Some(ref driver_specific_flags_to_change) = self.driver_specific_flags_to_change
		{
			let all_string_sets = validate(network_device_input_output_control.get_all_string_sets(), CouldNotGetAllStringSets)?;
			validate(network_device_input_output_control.set_private_flags(&all_string_sets, driver_specific_flags_to_change), CouldNotChangeDriverSpecificFlags)
		}
		
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
		
		if Some(generic_receive_offload_flush_timeout_in_nanoseconds) = self.generic_receive_offload_flush_timeout_in_nanoseconds
		{
			self.network_interface_name.set_generic_receive_offload_flush_timeout_in_nanoseconds(sys_path, generic_receive_offload_flush_timeout_in_nanoseconds).map_err(CouldNotSetGenericReceiveOffloadTimeout)
		}
		
		Ok((channels, pending_queue_depths))
	}
}
