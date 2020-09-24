// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit queue network device diagnostic.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceTransmitQueueDiagnostic
{
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub traffic_class: DiagnosticUnobtainableResult<Option<TransmitQueueTrafficClass>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub maximum_rate_in_megabits_per_second: DiagnosticUnobtainableResult<Option<NonZeroU32>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub number_of_timeout_events: DiagnosticUnobtainableResult<usize>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub transmit_packet_steering_hyper_thread_affinity: DiagnosticUnobtainableResult<Option<HyperThreads>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub transmit_packet_steering_receive_queue_affinity: DiagnosticUnobtainableResult<QueueIdentifiers>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub byte_limits_hold_time: DiagnosticUnobtainableResult<Milliseconds>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub number_of_bytes_inflight: DiagnosticUnobtainableResult<number_of_bytes_inflight>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub minimum_current_byte_limit: DiagnosticUnobtainableResult<Option<NonZeroU64>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub current_byte_limit: DiagnosticUnobtainableResult<Option<NonZeroU64>>,
	
	/// See detail in `Documentation/ABI/testing/sysfs-class-net-queues` in Linux source.
	pub maximum_current_byte_limit: DiagnosticUnobtainableResult<NonZeroU64>,
}

impl NetworkDeviceTransmitQueueDiagnostic
{
	#[inline(always)]
	fn gather(sys_path: &SysPath, transmit_sysfs_queue: &TransmitSysfsQueue) -> Self
	{
		Self
		{
			traffic_class: transmit_sysfs_queue.traffic_class(sys_path).map_err(DiagnosticUnobtainable::from),
			
			maximum_rate_in_megabits_per_second: transmit_sysfs_queue.maximum_rate_in_megabits_per_second(sys_path).map_err(DiagnosticUnobtainable::from),
			
			number_of_timeout_events: transmit_sysfs_queue.number_of_timeout_events(sys_path).map_err(DiagnosticUnobtainable::from),
			
			transmit_packet_steering_hyper_thread_affinity: transmit_sysfs_queue.transmit_packet_steering_hyper_thread_affinity(sys_path).map_err(DiagnosticUnobtainable::from),
			
			transmit_packet_steering_receive_queue_affinity: transmit_sysfs_queue.transmit_packet_steering_receive_queue_affinity(sys_path).map_err(DiagnosticUnobtainable::from),
			
			byte_limits_hold_time: transmit_sysfs_queue.byte_limits_hold_time(sys_path).map_err(DiagnosticUnobtainable::from),
			
			number_of_bytes_inflight: transmit_sysfs_queue.number_of_bytes_inflight(sys_path).map_err(DiagnosticUnobtainable::from),
			
			minimum_current_byte_limit: transmit_sysfs_queue.minimum_current_byte_limit(sys_path).map_err(DiagnosticUnobtainable::from),
			
			current_byte_limit: transmit_sysfs_queue.current_byte_limit(sys_path).map_err(DiagnosticUnobtainable::from),
			
			maximum_current_byte_limit: transmit_sysfs_queue.maximum_current_byte_limit(sys_path).map_err(DiagnosticUnobtainable::from),
		}
	}
}
