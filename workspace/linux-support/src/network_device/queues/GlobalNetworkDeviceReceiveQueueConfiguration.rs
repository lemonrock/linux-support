// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global network device configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalNetworkDeviceReceiveQueueConfiguration
{
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	#[serde(default)] pub receive_packet_steering_affinity: Option<HyperThreads>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	#[serde(default)] pub receive_flow_steering_table_count: Option<usize>,
}

impl GlobalNetworkDeviceReceiveQueueConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, receive_sysfs_queue: &ReceiveSysfsQueue) -> Result<(), GlobalNetworkDeviceConfigurationError>
	{
		use self::GlobalNetworkDeviceConfigurationError::*;
		
		if let Some(ref receive_packet_steering_affinity) = self.receive_packet_steering_affinity
		{
			receive_sysfs_queue.set_receive_packet_steering_affinity(sys_path, receive_packet_steering_affinity).map_err(CouldNotSetPerReceiveQueueReceivePacketSteeringAffinity)?
		}
		
		if let Some(receive_packet_steering_flow_table_count) = self.receive_flow_steering_table_count
		{
			receive_sysfs_queue.set_receive_packet_steering_flow_table_count(sys_path, receive_packet_steering_flow_table_count).map_err(CouldNotSetPerReceiveQueueReceivePacketSteeringAffinity)?
		}
		
		Ok(())
	}
	
	/// `receive_packet_steering_affinity` is only really useful if RSS is unsupported.
	/// It can be used with it, though; see the second answer in <https://stackoverflow.com/questions/44958511/what-is-the-main-difference-between-rss-rps-and-rfs>.
	#[inline(always)]
	pub(crate) fn use_receive_side_scaling_if_possible(receive_packet_steering_affinity: Option<HyperThread>, receive_flow_steering_table_count: usize) -> Self
	{
		Self
		{
			receive_packet_steering_affinity: receive_packet_steering_affinity.map(HyperThreads::for_one),
			
			receive_flow_steering_table_count: Some(receive_flow_steering_table_count),
		}
	}
}
