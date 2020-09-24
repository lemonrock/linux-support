// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive queue network device diagnostic.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceReceiveQueueDiagnostic
{
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub receive_packet_steering_affinity: DiagnosticUnobtainableResult<HyperThreads>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub receive_packet_steering_flow_table_count: DiagnosticUnobtainableResult<usize>,
}

impl NetworkDeviceReceiveQueueDiagnostic
{
	#[inline(always)]
	fn gather(sys_path: &SysPath, receive_sysfs_queue: &ReceiveSysfsQueue) -> Self
	{
		Self
		{
			receive_packet_steering_affinity: receive_sysfs_queue.receive_packet_steering_affinity(sys_path).map_err(DiagnosticUnobtainable::from),
			
			receive_packet_steering_flow_table_count: receive_sysfs_queue.receive_packet_steering_flow_table_count(sys_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
