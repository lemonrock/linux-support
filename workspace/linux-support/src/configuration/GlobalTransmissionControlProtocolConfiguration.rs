// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global Transmission Control Protocol (TCP) configuration.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalTransmissionControlProtocolConfiguration
{
	/// Global TCP minimum, default and maximum send buffer size.
	///
	/// Requires root.
	pub tcp_global_minimum_default_and_maximum_send_buffer_size_in_bytes: Option<(SendBufferSizeInBytes, SendBufferSizeInBytes, SendBufferSizeInBytes)>,
	
	/// Global TCP minimum, default and maximum receive buffer size.
	///
	/// Requires root.
	pub tcp_global_minimum_default_and_maximum_receive_buffer_size_in_bytes: Option<(ReceiveBufferSizeInBytes, ReceiveBufferSizeInBytes, ReceiveBufferSizeInBytes)>,
	
	/// Default is 75.
	///
	/// Requires root.
	pub tcp_keep_alive_interval_seconds: Option<KeepAliveIntervalSeconds>,
	
	/// Default is 7200.
	///
	/// Requires root.
	pub tcp_idles_before_keep_alive_seconds: Option<IdlesBeforeKeepAliveSeconds>,
	
	/// Default is 9.
	///
	/// Requires root.
	pub tcp_maximum_keep_alive_probes: Option<MaximumKeepAliveProbes>,
	
	/// Default is 60.
	///
	/// Requires root.
	pub tcp_finish_timeout_seconds: Option<FinishTimeoutSeconds>,
	
	/// Default is 6.
	///
	/// Requires root.
	pub tcp_maximum_syn_retransmits: Option<MaximumSynRetransmits>,
	
	/// Default is 5.
	///
	/// Requires root.
	pub tcp_maximum_syn_ack_retransmits: Option<MaximumSynAckRetransmits>,
	
	/// Default is 128.
	///
	/// Requires root.
	///
	/// This parameter is the queue size of open tcp connections awaiting an ACK packet to complete the 3-way handshake.
	/// In the event of a synflood DOS attack, this queue can fill up pretty quickly, at which point TCP SYN cookies will kick in allowing your system to continue to respond to legitimate traffic, and allowing you to gain access to block malicious IPs.
	pub tcp_maximum_syn_back_log: Option<BackLog>,
	
	/// Default is 4096.
	///
	/// Requires root.
	pub tcp_maximum_orphans: Option<NumberOfSockets>,
	
	/// Default is 4096.
	///
	/// Requires root.
	pub tcp_maximum_time_wait: Option<NumberOfSockets>,
	
	/// Default is `cubic`.
	///
	/// However `bbr` is better for HTTP/2 prioritization.
	///
	/// Requires root.
	pub tcp_congestion_control_algorithm: Option<CongestionControlAlgorithm>,
	
	/// Default is 3.
	///
	/// Requires root.
	pub tcp_retries_1: Option<Retries1>,
	
	/// Default is 15.
	///
	/// Requires root.
	pub tcp_retries_2: Option<Retries2>,
	
	/// Default is 0.
	///
	/// Requires root.
	pub tcp_retries_orphan: Option<RetriesOrphan>,
	
	/// Default is 3.
	///
	/// Requires root.
	pub tcp_reordering_threshold: Option<ReorderingThreshold>,
	
	/// Requires root.
	pub tcp_memory_pressure: Option<MemoryPressure>,

	/// See <https://patchwork.ozlabs.org/project/netdev/patch/20190614232221.248392-5-edumazet@google.com/> for rationale.
	///
	/// Default is false; setting this to true may increase performance on heavily loaded large systems.
	///
	/// Requires root.
	pub tcp_high_order_allocations: Option<bool>,
}

impl GlobalTransmissionControlProtocolConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalTransmissionControlProtocolConfigurationError>
	{
		use self::GlobalTransmissionControlProtocolConfigurationError::*;
		
		set_value(proc_path, SendBufferSizeInBytes::set_global_tcp_minimum_default_and_maximum, self.tcp_global_minimum_default_and_maximum_send_buffer_size_in_bytes, CouldNotChangeGlobalMinimumDefaultAndMaximumSendBufferSize)?;
		set_value(proc_path, ReceiveBufferSizeInBytes::set_global_tcp_minimum_default_and_maximum, self.tcp_global_minimum_default_and_maximum_receive_buffer_size_in_bytes, CouldNotChangeGlobalMinimumDefaultAndMaximumReceiveBufferSize)?;
		instance_set_value(proc_path, KeepAliveIntervalSeconds::set_global_default, self.tcp_keep_alive_interval_seconds, CouldNotChangeGlobalDefaultKeepAliveIntervalSeconds)?;
		instance_set_value(proc_path, IdlesBeforeKeepAliveSeconds::set_global_default, self.tcp_idles_before_keep_alive_seconds, CouldNotChangeGlobalDefaultIdlesBeforeKeepAliveSeconds)?;
		instance_set_value(proc_path, MaximumKeepAliveProbes::set_global_default, self.tcp_maximum_keep_alive_probes, CouldNotChangeGlobalDefaultMaximumKeepAliveProbes)?;
		instance_set_value(proc_path, FinishTimeoutSeconds::set_global_default, self.tcp_finish_timeout_seconds, CouldNotChangeGlobalDefaultFinishTimeout)?;
		instance_set_value(proc_path, MaximumSynRetransmits::set_global_maximum, self.tcp_maximum_syn_retransmits, CouldNotChangeGlobalDefaultMaximumSynRetransmits)?;
		instance_set_value(proc_path, MaximumSynAckRetransmits::set_global_maximum, self.tcp_maximum_syn_ack_retransmits, CouldNotChangeGlobalDefaultMaximumSynAckRetransmits)?;
		instance_set_value(proc_path, BackLog::set_global_maximum_syn, self.tcp_maximum_syn_back_log, CouldNotChangeGlobalMaximumSynBackLog)?;
		instance_set_value(proc_path, NumberOfSockets::set_global_maximum_orphans, self.tcp_maximum_orphans, CouldNotChangeGlobalMaximumOrphans)?;
		instance_set_value(proc_path, NumberOfSockets::set_global_maximum_time_wait, self.tcp_maximum_time_wait, CouldNotChangeGlobalMaximumTimeWait)?;
		instance_set_value(proc_path, CongestionControlAlgorithm::set_global_default, self.tcp_congestion_control_algorithm, CouldNotChangeGlobalDefaultCongestionControlAlgorithm)?;
		instance_set_value(proc_path, Retries1::set_global_default, self.tcp_retries_1, CouldNotChangeGlobalDefaultRetries1)?;
		instance_set_value(proc_path, Retries2::set_global_default, self.tcp_retries_2, CouldNotChangeGlobalDefaultRetries2)?;
		instance_set_value(proc_path, RetriesOrphan::set_global_default, self.tcp_retries_orphan, CouldNotChangeGlobalDefaultRetriesOrphan)?;
		instance_set_value(proc_path, ReorderingThreshold::set_global_maximum, self.tcp_reordering_threshold, CouldNotChangeGlobalDefaultReorderingThreshold)?;
		instance_set_value(proc_path, MemoryPressure::set_global, self.tcp_memory_pressure.as_ref(), CouldNotChangeMemoryPressure)?;
		set_proc_sys_net_core_value(proc_path, "high_order_alloc_disable", self.tcp_high_order_allocations, CouldNotChangeHighOrderAllocations)?;
		
		Ok(())
	}
}
