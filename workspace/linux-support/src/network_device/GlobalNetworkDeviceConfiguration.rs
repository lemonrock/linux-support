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
	
	/// Transmission queue length, unrelated apparently to `PendingQueueDepths.transmit_pending_queue_depth`.
	///
	/// Default on Linux is 1000.
	/// A value of 128 has been suggested for some improvement in reducing bufferbloat.
	#[serde(default)] pub transmission_queue_length: Option<u32>,
	
	/// Maximum Transmission Unit (MTU).
	///
	/// Does not normally need to change.
	#[serde(default)] pub maximum_transmission_unit: Option<MaximumTransmissionUnit>,
	
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
	///
	/// Changing the number of channels is not possible if an eXpress Data Path (XDP) program is attached.
	/// Changing the number of channels is not possible if Intel Flow Director (fdir) rules are active.
	/// Changing the number of channels is not possible for Intel i40e if traffic classes (TCs) are configured through using the Multiqueue Priority Qdisc (Offloaded Hardware QOS), [MQPRIO](https://www.man7.org/linux/man-pages/man8/tc-mqprio.8.html).
	#[serde(default)] pub maximize_number_of_channels: bool,

	/// Maximize pending queue depths?
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

/*
	TODO:
	the CPU which handles the interrupt will also process the packet unless:-
		/sys/class/net/<device>/queues/<rx-queue>/rps_cpus - a cpu bitmap - is changed - to enable RPS - receive packet steering - which is a software tech, I think.
		See https://www.suse.com/support/kb/doc/?id=000018430
	
	On NUMA machines, best performance can be achieved by configuring RPS to use the CPUs on the same NUMA node as the interrupt IRQ for the interface's receive queue.
 */


// https://www.kernel.org/doc/html/v5.8/networking/scaling.html
/*

TODO: Receive Queue => CPU

--config-nfc / --config-ntuple
	do_srxclass
		"rx-flow-hash"
		"flow-type"
			do_srxntuple
		"delete"
		
--per-queue ... eg coalesce

Rework set ring params, eg Intel i40e returns EINVAL for values outside of the range I40E_MIN_NUM_DESCRIPTORS ..= I40E_MAX_NUM_DESCRIPTORS
	eg Intel i40e does not suppport rx_mini_pending or rx_jumbo_pending

Indirection RSS hash table
This table can be configured. For example, to make sure all traffic goes only to CPUs #0-#5 (the first NUMA node in our setup), we run:
client$ sudo ethtool -X eth2 weight 1 1 1 1 1 1 0 0 0 0 0
Finally, ensure the interrupts of multiqueue network cards are evenly distributed between CPUs. The irqbalance service is stopped and the interrupts are manually assigned. For simplicity let's pin the RX queue #0 to CPU #0, RX queue #1 to CPU #1 and so on.


We currently use the following in GlobalSchedulingConfiguration


MSI-X - Message Signal Interrupts (Extended).

When using MSI-X, an IRQ is raised for the RX queue the packet was written on.
This IRQ is then mapped to a CPU (or set of CPUs)


client$ (let CPU=0; cd /sys/class/net/eth0/device/msi_irqs/;
         for IRQ in *; do
            echo $CPU > /proc/irq/$IRQ/smp_affinity_list
            let CPU+=1
         done)
NOTE: /sys/class/net/eth2/device/msi_irqs may not exist, in which case:-

	grep eth0 /proc/interrupts
	32:	0	140	45	850264	PCI-MSI-edge	eth0
	
	This means eth0 has assigned IRQ number 32.
	There may be multiple lines.
	The device may not exist at all (eg a Parallels VM)
	
	Change /proc/irq/32/smp_affinity to change the CPUs dealing with that IRQ
		- The list of CPUs should be on the same NUMA node as the eth0 device (ie check eth0's PCI device).

	Other lines might look like this if MSI-X is available:-
	            CPU0       CPU1       CPU2       CPU3
	  65:          1          0          0          0 IR-PCI-MSI-edge      eth0
	  66:  863649703          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-0
	  67:  986285573          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-1
	  68:         45          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-2
	  69:        394          0          0          0 IR-PCI-MSI-edge      eth0-TxRx-3

This is because each RX queue can have its own hardware interrupt assigned if using MSI-X.


Inputs into a weighting algorithm

	- number of receive queues, number_of_receive_queues (eg 2, 11)
	- indirection_table_size (eg 128) - this is the denominator.
	
	
	


https://docs.gz.ro/tuning-network-cards-on-linux.html
https://blog.cloudflare.com/how-to-achieve-low-latency/
https://serverfault.com/questions/772380/how-to-tell-if-nic-has-multiqueue-enabled (multiqueue - more than one rx or tx queue).
https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/


You can adjust the net_rx_action budget, which determines how much packet processing can be spent among all NAPI structures registered to a CPU
/proc/sys/net/core/netdev_budget
	- default is 300.

Tuning: Enabling accelerated RFS (aRFS)

Assuming that your NIC and driver support it, you can enable accelerated RFS by enabling and configuring a set of things:

    Have RPS enabled and configured.
    	So, for eth0 and receive queue 0, you would modify the file: /sys/class/net/eth0/queues/rx-0/rps_cpus with a hexadecimal number indicating which CPUs should process packets from eth0’s receive queue 0.
    	https://github.com/torvalds/linux/blob/v3.13/Documentation/networking/scaling.txt#L160-L164
    Have RFS enabled and configured.
    	Have RPS enabled and configured.
    	RFS keeps track of a global hash table of all flows and the size of this hash table can be adjusted by setting the net.core.rps_sock_flow_entries sysctl.
    	Next, you can also set the number of flows per RX queue by writing this value to the sysfs file named rps_flow_cnt for each RX queue.
		Example: increase the number of flows for RX queue 0 on eth0 to 2048.
		$ sudo bash -c 'echo 2048 > /sys/class/net/eth0/queues/rx-0/rps_flow_cnt'
    Your kernel has CONFIG_RFS_ACCEL enabled at compile time. The Ubuntu kernel 3.13.0 does.
    Have ntuple support enabled for the device, as described previously. You can use ethtool to verify that ntuple support is enabled for the device.
    Configure your IRQ settings to ensure each RX queue is handled by one of your desired network processing CPUs.

Once the above is configured, accelerated RFS will be used to automatically move data to the RX queue tied to a CPU core that is processing data for that flow and you won’t need to specify an ntuple filter rule manually for each flow.


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

impl GlobalNetworkDeviceConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, network_interface_name: &NetworkInterfaceName) -> Result<(Channels, PendingQueueDepths), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		#[inline(always)]
		fn validate<A, E>(network_device_input_output_control: &NetworkDeviceInputOutputControl, change_result: impl FnOnce(&NetworkDeviceInputOutputControl) -> Result<Option<A>, E>, error: impl FnOnce(E) -> GlobalNetworkDeviceConfigurationError) -> Result<A, GlobalNetworkDeviceConfigurationError>
		{
			change(network_device_input_output_control).map_err(error)?.ok_or(NetworkDeviceDoesNotExist(network_device_input_output_control.network_interface_name()))
		}
		
		let network_device_input_output_control = NetworkDeviceInputOutputControl::new(Cow::Borrowed(network_interface_name))?;
		
		if let Some(driver_message_level) = self.driver_message_level
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::set_driver_message_level(driver_message_level), CouldNotSetDriverMessageLevel)?
		}
		
		if let Some(transmission_queue_length) = self.transmission_queue_length
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_transmission_queue_length(transmission_queue_length), CouldNotSetTransmissionQueueLength)?;
		}
		
		if let Some(maximum_transmission_unit) = self.maximum_transmission_unit
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_maximum_transmission_unit(maximum_transmission_unit), CouldNotSetMaximumTransmissionUnit)?;
		}
		
		if let Some(forward_error_correction) = self.forward_error_correction
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_forward_error_correction(forward_error_correction), CouldNotSetForwardErrorConnection)?
		}
		
		if let Some(pause_configuration) = self.pause_configuration
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_pause(pause_configuration), CouldNotSetPause)?
		}
		
		if let Some(ref energy_efficient_ethernet) = self.energy_efficient_ethernet
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_energy_efficient_ethernet(energy_efficient_ethernet), CouldNotSetForwardErrorConnection)?
		}
		
		if self.disable_wake_on_lan
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sdisable_wake_on_lan(), CouldNotDisableWakeOnLan)?
		}
		
		validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_features(FeatureGroupChoice::iter(&self.feature_group_choices)), CouldNotChangeFeatures)?;
		
		if let Some(ref driver_specific_flags_to_change) = self.driver_specific_flags_to_change
		{
			let all_string_sets = validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sget_all_string_sets(), CouldNotGetAllStringSets)?;
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_private_flags(&all_string_sets, driver_specific_flags_to_change), CouldNotChangeDriverSpecificFlags)
		}
		
		for tunable in self.tunables.iter()
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_tunable(table), CouldNotChangeTunable)?
		}
		
		if let Some(ref coalesce_configuration) = self.coalesce_configuration
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::schange_coalesce_configuration(coalesce_configuration), CouldNotChangeCoalesceConfiguration)?
		}
		
		let channels = validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::smaximize_number_of_channels(self.maximize_number_of_channels), CouldNotMaximizeChannels)?;
		
		let pending_queue_depths = validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::smaximize_receive_ring_queues_and_transmit_ring_queue_depths(self.maximize_pending_queue_depths), CouldNotMaximizePendingQueueDepths)?;
		
		if let Some(ref receive_side_scaling_hash_configuration) = self.receive_side_scaling_hash_configuration
		{
			validate(&network_device_input_output_control, NetworkDeviceInputOutputControl::sset_configured_hash_settings(None, receive_side_scaling_hash_configuration), CouldNotConfigureReceiveSideScalingHashConfiguration)?
		}
		
		if Some(generic_receive_offload_flush_timeout_in_nanoseconds) = self.generic_receive_offload_flush_timeout_in_nanoseconds
		{
			self.network_interface_name.set_generic_receive_offload_flush_timeout_in_nanoseconds(sys_path, generic_receive_offload_flush_timeout_in_nanoseconds).map_err(CouldNotSetGenericReceiveOffloadTimeout)
		}
		
		Ok((channels, pending_queue_depths))
	}
}
