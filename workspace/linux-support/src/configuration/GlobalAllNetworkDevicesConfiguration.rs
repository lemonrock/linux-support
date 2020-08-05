// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Network device configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalAllNetworkDevicesConfiguration
{
	/// The maximum number of packets that kernel can handle on a NAPI interrupt, per-HyperThread.
	/// If a driver supports receive offloading (GRO or LRO) then the aggregated packet is considered to be one packet.
	///
	/// Default is 64.
	///
	/// Changes `/proc/sys/net/core/dev_weight`.
	pub weight: Option<NonZeroU32>,
	
	/// This parameter influences the proportion of the configured `maximum_number_of_packets_in_one_napi_polling_cycle` that is spent on RPS based packet processing during RX softirq cycles; it scales the value of `weight`.
	///
	/// Default is 1.
	///
	/// Changes `/proc/sys/net/core/dev_weight_rx_bias`.
	pub weight_receive_scalar: Option<NonZeroU32>,
	
	/// This parameter influences the proportion of the configured `maximum_number_of_packets_in_one_napi_polling_cycle` that is spent on a TX softirq cycle; it scales the value of `weight`.
	///
	/// Default is 1.
	///
	/// Changes `/proc/sys/net/core/dev_weight_tx_bias`.
	pub weight_transmit_scalar: Option<NonZeroU32>,
	
	/// Maximum number of packets taken from all interfaces in one polling cycle (NAPI poll).
	///
	/// In one polling cycle interfaces which are registered to polling are probed in a round-robin manner.
	///
	/// A polling cycle may not exceed `maximum_time_in_microseconds_for_one_napi_polling_cycle` microseconds, even if `budget` has not been exhausted.
	///
	/// Default is 300.
	///
	/// Changes `/proc/sys/net/core/netdev_budget`.
	pub maximum_number_of_packets_in_one_napi_polling_cycle: Option<NonZeroU32>,
	
	/// Maximum number of microseconds in one NAPI polling cycle.
	///
	/// Polling will exit when either netdev_budget_usecs have elapsed during the/ poll cycle or the number of packets processed reaches `maximum_number_of_packets_in_one_napi_polling_cycle`.
	///
	/// Default is 2000.
	///
	/// Changes `/proc/sys/net/core/netdev_budget_usecs`.
	pub maximum_time_in_microseconds_for_one_napi_polling_cycle: Option<NonZeroU32>,
	
	/// Maximum number of received packets queued when a network device receives packets faster than kernel can process them.
	///
	/// Changes `/proc/sys/net/core/netdev_max_backlog`.
	pub maximum_back_log_of_received_packets_in_the_kernel: Option<NonZeroU32>,
	
	/// Set this to false for a small performance gain if using timestamped packets in exchange for a loss of precision of timestamps.
	///
	/// Default is true.
	///
	/// Changes `/proc/sys/net/core/netdev_tstamp_prequeue`.
	pub prequeue_timestamp_packets: Option<bool>,

	/// This is ordinarily `17` but needs to be tweaked for performance on some set ups.
	///
	/// It's a Linux hack.
	///
	/// Changes `/proc/sys/net/core/max_skb_frags`.
	pub maximum_fragments_in_a_socket_buffer: Option<NonZeroU8>,

	/// This is ordinarily `8` but `16` may give better results on some set ups.
	///
	/// Changes `/proc/sys/net/core/gro_normal_batch`.
	pub number_of_socket_buffer_lists_to_free_in_a_generic_receive_offload_batch: Option<NonZeroU8>,
}

impl GlobalAllNetworkDevicesConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalAllNetworkDevicesConfigurationError>
	{
		fn write_value(proc_path: &ProcPath, file_name: &str, value: NonZeroU32) -> io::Result<()>
		{
			proc_path.sys_net_core_file_path(file_name).write_value(UnpaddedDecimalInteger(value))
		}
		
		use self::GlobalAllNetworkDevicesConfigurationError::*;
		
		set_proc_sys_net_core_value(proc_path, "dev_weight", self.weight.map(UnpaddedDecimalInteger), CouldNotChangeWeight)?;
		set_proc_sys_net_core_value(proc_path, "dev_weight_rx_bias", self.weight_receive_scalar.map(UnpaddedDecimalInteger), CouldNotChangeWeightReceiveScalar)?;
		set_proc_sys_net_core_value(proc_path, "dev_weight_tx_bias", self.weight_transmit_scalar.map(UnpaddedDecimalInteger), CouldNotChangeWeightTransmitScalar)?;
		set_proc_sys_net_core_value(proc_path, "netdev_budget", self.maximum_number_of_packets_in_one_napi_polling_cycle.map(UnpaddedDecimalInteger), CouldNotChangeMaximumNumberOfPacketsInOneNapiPollingCycle)?;
		set_proc_sys_net_core_value(proc_path, "netdev_budget_usecs", self.maximum_time_in_microseconds_for_one_napi_polling_cycle.map(UnpaddedDecimalInteger), CouldNotChangeMaximumTimeForOneNapiPollingCycle)?;
		set_proc_sys_net_core_value(proc_path, "netdev_max_backlog", self.maximum_back_log_of_received_packets_in_the_kernel.map(UnpaddedDecimalInteger), CouldNotChangeMaximumBackLogOfReceivedPacketsInTheKernel)?;
		set_proc_sys_net_core_value(proc_path, "netdev_tstamp_prequeue", self.prequeue_timestamp_packets, CouldNotChangeTimestampPrequeue)?;
		set_proc_sys_net_core_value(proc_path, "max_skb_frags", self.maximum_fragments_in_a_socket_buffer.map(UnpaddedDecimalInteger), CouldNotChangeMaximumFragmentsInASocketBuffer)?;
		set_proc_sys_net_core_value(proc_path, "gro_normal_batch", self.number_of_socket_buffer_lists_to_free_in_a_generic_receive_offload_batch.map(UnpaddedDecimalInteger), CouldNotChangeNumberOfSocketBufferListsToFreeInAGenericReceiveOffloadBatch)?;
		
		Ok(())
	}
}
