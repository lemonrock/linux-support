// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkConfiguration
{
	/// Global maximum send buffer size.
	///
	/// Requires root.
	pub global_maximum_send_buffer_size_in_bytes: Option<SendBufferSizeInBytes>,
	
	/// Global default send buffer size.
	///
	/// Requires root.
	pub global_default_send_buffer_size_in_bytes: Option<SendBufferSizeInBytes>,
	
	/// Global TCP minimum, default and maximum send buffer size.
	///
	/// Requires root.
	pub global_tcp_minimum_default_and_maximum_send_buffer_size_in_bytes: Option<(SendBufferSizeInBytes, SendBufferSizeInBytes, SendBufferSizeInBytes)>,
	
	/// Global maximum receive buffer size.
	///
	/// Requires root.
	pub global_maximum_receive_buffer_size_in_bytes: Option<ReceiveBufferSizeInBytes>,
	
	/// Global default receive buffer size.
	///
	/// Requires root.
	pub global_default_receive_buffer_size_in_bytes: Option<ReceiveBufferSizeInBytes>,
	
	/// Global TCP minimum, default and maximum receive buffer size.
	///
	/// Requires root.
	pub global_tcp_minimum_default_and_maximum_receive_buffer_size_in_bytes: Option<(ReceiveBufferSizeInBytes, ReceiveBufferSizeInBytes, ReceiveBufferSizeInBytes)>,
	
	/// Default is 75.
	///
	/// Requires root.
	pub keep_alive_interval_seconds: Option<KeepAliveIntervalSeconds>,
	
	/// Default is 7200.
	///
	/// Requires root.
	pub idles_before_keep_alive_seconds: Option<IdlesBeforeKeepAliveSeconds>,
	
	/// Default is 9.
	///
	/// Requires root.
	pub maximum_keep_alive_probes: Option<MaximumKeepAliveProbes>,
	
	/// Default is 60.
	///
	/// Requires root.
	pub finish_timeout_seconds: Option<FinishTimeoutSeconds>,
	
	/// Default is 6.
	///
	/// Requires root.
	pub maximum_syn_retransmits: Option<MaximumSynRetransmits>,
	
	/// Default is 5.
	///
	/// Requires root.
	pub maximum_syn_ack_retransmits: Option<MaximumSynAckRetransmits>,
	
	/// Default is 4_294_967_295.
	///
	/// Set this to 16KB for HTTP/2 prioritization to work reliably.
	///
	/// Requires root.
	pub not_sent_low_water_in_bytes: Option<NotSentLowWaterInBytes>,
	
	/// Default is 128.
	///
	/// Requires root.
	pub maximum_back_log: Option<BackLog>,
	
	/// Default is 128.
	///
	/// Requires root.
	///
	/// This parameter is the queue size of open tcp connections awaiting an ACK packet to complete the 3-way handshake.
	/// In the event of a synflood DOS attack, this queue can fill up pretty quickly, at which point TCP SYN cookies will kick in allowing your system to continue to respond to legitimate traffic, and allowing you to gain access to block malicious IPs.
	pub maximum_syn_back_log: Option<BackLog>,
	
	/// Default is 4096.
	///
	/// Requires root.
	pub maximum_orphans: Option<NumberOfSockets>,
	
	/// Default is 4096.
	///
	/// Requires root.
	pub maximum_time_wait: Option<NumberOfSockets>,
	
	/// Default is `cubic`.
	///
	/// However `bbr` is better for HTTP/2 prioritization.
	///
	/// Requires root.
	pub congestion_control_algorithm: Option<CongestionControlAlgorithm>,
	
	/// Default is `pfifo_fast`.
	///
	/// However `fq` is better for HTTP/2 prioritization.
	///
	/// Requires root.
	pub queuing_discipline_algorithm: Option<QueuingDisciplineAlgorithm>,
	
	/// Default is 3.
	///
	/// Requires root.
	pub retries_1: Option<Retries1>,
	
	/// Default is 15.
	///
	/// Requires root.
	pub retries_2: Option<Retries2>,
	
	/// Default is 0.
	///
	/// Requires root.
	pub retries_orphan: Option<RetriesOrphan>,
	
	/// Default is 3.
	///
	/// Requires root.
	pub reordering_threshold: Option<ReorderingThreshold>,
	
	/// Requires root.
	pub memory_pressure: Option<MemoryPressure>,
	
	/// Default is 0!
	///
	/// A better value might be 32768.
	///
	/// Requires root.
	pub maximum_receive_packet_steering_flows_per_hyper_thread: Option<ReceivePacketSteeringFlowsPerHyperThread>,
	
	/// By default, Linux sets auto corking to on (true).
	///
	/// Requires root.
	pub adjust_auto_corking: Option<bool>,
}

/*

https://blog.packagecloud.io/eng/2016/06/22/monitoring-tuning-linux-networking-stack-receiving-data/

RPS:
TODO: Modify file /sys/class/net/eth0/queues/rx-0/rps_cpus  and insert a CPU bitmask

RFS:
TODO: Modify file  /sys/class/net/eth0/queues/rx-0/rps_flow_cnt
echo 2048 > /sys/class/net/eth0/queues/rx-0/rps_flow_cnt

 */


impl GlobalNetworkConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalNetworkConfigurationError>
	{
		use self::GlobalNetworkConfigurationError::*;
		
		#[inline(always)]
		fn instance_set_value<'a, Value, Cause, Error: error::Error>(proc_path: &ProcPath, function: impl FnOnce(Value, &ProcPath) -> Result<(), Cause>, value: Option<Value>, error: impl FnOnce(Cause) -> Error) -> Result<(), Error>
		{
			if let Some(value) = value
			{
				return function(value, proc_path).map_err(error)
			}
			
			Ok(())
		}
		
		
		instance_set_value(proc_path, SendBufferSizeInBytes::set_global_maximum, self.global_maximum_send_buffer_size_in_bytes, CouldNotChangeGlobalMaximumSendBufferSize)?;
		instance_set_value(proc_path, SendBufferSizeInBytes::set_global_default, self.global_default_send_buffer_size_in_bytes, CouldNotChangeGlobalDefaultSendBufferSize)?;
		set_value(proc_path, SendBufferSizeInBytes::set_global_tcp_minimum_default_and_maximum, self.global_tcp_minimum_default_and_maximum_send_buffer_size_in_bytes, CouldNotChangeGlobalTcpMinimumDefaultAndMaximumSendBufferSize)?;
		
		instance_set_value(proc_path, ReceiveBufferSizeInBytes::set_global_maximum, self.global_maximum_receive_buffer_size_in_bytes, CouldNotChangeGlobalMaximumReceiveBufferSize)?;
		instance_set_value(proc_path, ReceiveBufferSizeInBytes::set_global_default, self.global_default_receive_buffer_size_in_bytes, CouldNotChangeGlobalDefaultReceiveBufferSize)?;
		set_value(proc_path, ReceiveBufferSizeInBytes::set_global_tcp_minimum_default_and_maximum, self.global_tcp_minimum_default_and_maximum_receive_buffer_size_in_bytes, CouldNotChangeGlobalTcpMinimumDefaultAndMaximumReceiveBufferSize)?;
		
		instance_set_value(proc_path, KeepAliveIntervalSeconds::set_global_default, self.keep_alive_interval_seconds, CouldNotChangeGlobalDefaultKeepAliveIntervalSeconds)?;
		instance_set_value(proc_path, IdlesBeforeKeepAliveSeconds::set_global_default, self.idles_before_keep_alive_seconds, CouldNotChangeGlobalDefaultIdlesBeforeKeepAliveSeconds)?;
		instance_set_value(proc_path, MaximumKeepAliveProbes::set_global_default, self.maximum_keep_alive_probes, CouldNotChangeGlobalDefaultMaximumKeepAliveProbes)?;
		
		instance_set_value(proc_path, FinishTimeoutSeconds::set_global_default, self.finish_timeout_seconds, CouldNotChangeGlobalDefaultFinishTimeout)?;
		
		instance_set_value(proc_path, MaximumSynRetransmits::set_global_maximum, self.maximum_syn_retransmits, CouldNotChangeGlobalDefaultMaximumSynRetransmits)?;
		instance_set_value(proc_path, MaximumSynAckRetransmits::set_global_maximum, self.maximum_syn_ack_retransmits, CouldNotChangeGlobalDefaultMaximumSynAckRetransmits)?;
		
		instance_set_value(proc_path, NotSentLowWaterInBytes::set_global_default, self.not_sent_low_water_in_bytes, CouldNotChangeGlobalDefaultNotSentLowWater)?;
		
		instance_set_value(proc_path, BackLog::set_global_maximum, self.maximum_back_log, CouldNotChangeGlobalMaximumBackLog)?;
		instance_set_value(proc_path, BackLog::set_global_maximum_syn, self.maximum_syn_back_log, CouldNotChangeGlobalMaximumSynBackLog)?;
		
		instance_set_value(proc_path, NumberOfSockets::set_global_maximum_orphans, self.maximum_orphans, CouldNotChangeGlobalMaximumOrphans)?;
		instance_set_value(proc_path, NumberOfSockets::set_global_maximum_time_wait, self.maximum_time_wait, CouldNotChangeGlobalMaximumTimeWait)?;
		
		instance_set_value(proc_path, CongestionControlAlgorithm::set_global_default, self.congestion_control_algorithm, CouldNotChangeGlobalDefaultCongestionControlAlgorithm)?;
		instance_set_value(proc_path, QueuingDisciplineAlgorithm::set_global_default, self.queuing_discipline_algorithm, CouldNotChangeGlobalDefaultQueuingDisciplineAlgorithm)?;
		
		instance_set_value(proc_path, Retries1::set_global_default, self.retries_1, CouldNotChangeGlobalDefaultRetries1)?;
		instance_set_value(proc_path, Retries2::set_global_default, self.retries_2, CouldNotChangeGlobalDefaultRetries2)?;
		instance_set_value(proc_path, RetriesOrphan::set_global_default, self.retries_orphan, CouldNotChangeGlobalDefaultRetriesOrphan)?;
		
		instance_set_value(proc_path, ReorderingThreshold::set_global_maximum, self.reordering_threshold, CouldNotChangeGlobalDefaultReorderingThreshold)?;
		
		instance_set_value(proc_path, MemoryPressure::set_global, self.memory_pressure.as_ref(), CouldNotChangeMemoryPressure)?;
		
		instance_set_value(proc_path, ReceivePacketSteeringFlowsPerHyperThread::set_global_maximum, self.maximum_receive_packet_steering_flows_per_hyper_thread, CouldNotChangeGlobalDefaultReceivePacketSteeringFlowsPerCpu)?;
		
		set_value(proc_path, set_auto_corking, self.adjust_auto_corking, CouldNotChangeAutoCorking)?;
		
		Ok(())
	}
}
