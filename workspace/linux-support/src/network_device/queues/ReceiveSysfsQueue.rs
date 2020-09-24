// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A receive queue reference to a folder `/sys/class/net/<network_interface_name>/queues/rx-<N>` where `N` is `queue_identifier`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveSysfsQueue<'a>
{
	network_interface_name: &'a NetworkInterfaceName,
	
	queue_identifier: QueueIdentifier,
}

impl<'a> SysfsQueue<'a> for ReceiveSysfsQueue<'a>
{
	const Prefix: &'static str = "rx";
	
	fn new(network_interface_name: &'a NetworkInterfaceName, queue_identifier: QueueIdentifier) -> Self
	{
		Self
		{
			network_interface_name,
			queue_identifier,
		}
	}
	
	#[inline(always)]
	fn network_interface_name(&self) -> &'a NetworkInterfaceName
	{
		self.network_interface_name
	}
	
	#[inline(always)]
	fn queue_identifier(&self) -> QueueIdentifier
	{
		self.queue_identifier
	}
}

impl<'a> ReceiveSysfsQueue<'a>
{
	
	/// Receive Packet Steering (RPS) affinity.
	///
	/// Default is `HyperThreads::empty()`.
	#[inline(always)]
	pub fn receive_packet_steering_affinity(&self, sys_path: &SysPath) -> io::Result<HyperThreads>
	{
		self.rps_cpus_file_path(sys_path).parse_comma_separated_bit_set().map(HyperThreads)
	}
	
	/// Set Receive Packet Steering (RPS) affinity.
	///
	/// Default is `HyperThreads::empty()`.
	#[inline(always)]
	pub fn set_receive_packet_steering_affinity(&self, sys_path: &SysPath, hyper_threads: &HyperThreads) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/rx-<queue_identifier/rps_cpus");
		
		let file_path = self.rps_cpus_file_path(sys_path);
		
		if file_path.exists()
		{
			let mask = IntoBitMask(hyper_threads);
			file_path.write_value(mask)
		}
		else
		{
			Ok(())
		}
	}
	
	/// Receive Packet Steering (RPS) flow table count.
	///
	/// The flow table count is the maximum number of Receive Packet Steering (RPS) flows for a receive queue.
	///
	/// Default is `0`.
	#[inline(always)]
	pub fn receive_packet_steering_flow_table_count(&self, sys_path: &SysPath) -> io::Result<usize>
	{
		self.rps_flow_cnt_file_path(sys_path).read_value()
	}
	
	/// Set Receive Packet Steering (RPS) flow table count.
	///
	/// The flow table count is the maximum number of Receive Packet Steering (RPS) flows for a receive queue.
	///
	/// Default is `0`.
	#[inline(always)]
	pub fn set_receive_packet_steering_flow_table_count(&self, sys_path: &SysPath, flow_table_count: usize) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/rx-<queue_identifier/rps_flow_cnt");
		
		let file_path = self.rps_flow_cnt_file_path(sys_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(flow_table_count))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn rps_flow_cnt_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.file_path(sys_path, "rps_flow_cnt")
	}
	
	#[inline(always)]
	fn rps_cpus_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.file_path(sys_path, "rps_cpus")
	}
}
