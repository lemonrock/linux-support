// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A transmit queue reference to a folder `/sys/class/net/<network_interface_name>/queues/tx-<N>` where `N` is `queue_identifier`.
///
/// The files `traffic_class` (which is only read only) seems unreadable on my test machine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitSysfsQueue<'a>
{
	network_interface_name: &'a NetworkInterfaceName,
	
	queue_identifier: QueueIdentifier,
}

impl<'a> SysfsQueue<'a> for TransmitSysfsQueue<'a>
{
	const Prefix: &'static str = "tx";
	
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

impl<'a> TransmitSysfsQueue<'a>
{
	/// Traffic class.
	///
	/// Returns `Ok(Some(_))` if successfully retrieved.
	///
	/// ***Only supported if the network device is multiqueue (ie has more than one transmit queue); if not supported, `Ok(None)` is returned`.
	/// Most virtual network devices are not multiqueue.
	pub fn traffic_class(&self, sys_path: &SysPath) -> io::Result<Option<TransmitQueueTrafficClass>>
	{
		match self.file_path(sys_path, "traffic_class").read_raw_without_line_feed()
		{
			Ok(value) =>
			{
				let value = &value[..];
				let mut bytes = value.split_bytes_n(2, b'-');
				
				Ok
				(
					Some
					(
						TransmitQueueTrafficClass
						{
							traffic_class: ParseNumber::parse_decimal_number(bytes.next().unwrap()).map_err(io_error_invalid_data)?,
						
							subordinate_device: match bytes.next()
							{
								Some(subordinate_device_bytes) => Some(ParseNumber::parse_decimal_number(subordinate_device_bytes).map_err(io_error_invalid_data)?),
								
								None => None,
							},
						}
					)
				)
			},
			
			Err(error) => if error.raw_os_error() == Some(ENOENT)
			{
				Ok(None)
			}
			else
			{
				Err(error)
			}
		}
	}
	
	/// Maximum rate.
	///
	/// Default is `None` (disabled).
	///
	/// Only available in Linux kernel compiled with `CONFIG_XPS`.
	///
	/// ***Not supported by some network devices, in which case `Ok(0)` (rather than `EOPNOTSUPP`)!***
	#[inline(always)]
	pub fn maximum_rate_in_megabits_per_second(&self, sys_path: &SysPath) -> io::Result<Option<NonZeroU32>>
	{
		self.tx_maxrate_file_path(sys_path).read_value()
	}
	
	/// Set maximum rate.
	///
	/// Default is `None` (disabled).
	///
	/// Only available in Linux kernel compiled with `CONFIG_XPS`.
	///
	/// Returns `Ok(true)` if successfully set.
	///
	/// ***Not supported by some network devices, in which case `Ok(false)` is returned.***.
	/// `Ok(false)` is also returned if the file does not exist.
	#[inline(always)]
	pub fn set_maximum_rate_in_megabits_per_second(&self, sys_path: &SysPath, maximum_rate_in_megabits_per_second: Option<NonZeroU32>) -> io::Result<bool>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/tx_maxrate");
		
		let file_path = self.tx_maxrate_file_path(sys_path);
		
		if file_path.exists()
		{
			let maximum_rate_in_megabits_per_second: u32 = unsafe { transmute(maximum_rate_in_megabits_per_second) };
			match file_path.write_value(UnpaddedDecimalInteger(maximum_rate_in_megabits_per_second))
			{
				Ok(()) => Ok(true),
				
				Err(error) => if error.raw_os_error() == Some(EOPNOTSUPP)
				{
					Ok(false)
				}
				else
				{
					Err(error)
				}
			}
		}
		else
		{
			Ok(false)
		}
	}
	/// Number of transmit timeout events.
	#[inline(always)]
	pub fn number_of_timeout_events(&self, sys_path: &SysPath) -> io::Result<usize>
	{
		self.file_path(sys_path, "tx_timeout").read_value()
	}
	
	/// Transmit Packet Steering (XPS) mapping of receive queues to this transmit queue.
	///
	/// Used as ?legacy alternative to receive queue affinity.
	///
	/// Default is disabled.
	///
	/// Returns `Ok(Some(_))` if successfully retrieved.
	///
	/// ***Only supported if the network device is multiqueue (ie has more than one transmit queue); if not supported, `Ok(None)` is returned`.
	/// Most virtual network devices are not multiqueue.
	#[inline(always)]
	pub fn transmit_packet_steering_hyper_thread_affinity(&self, sys_path: &SysPath) -> io::Result<Option<HyperThreads>>
	{
		match self.xps_cpus_file_path(sys_path).parse_comma_separated_bit_set()
		{
			Ok(bit_set) => Ok(Some(HyperThreads(bit_set))),
			
			Err(error) => if error.raw_os_error() == Some(ENOENT)
			{
				Ok(None)
			}
			else
			{
				Err(error)
			}
		}
	}
	
	/// Transmit Packet Steering (XPS) mapping of receive queues to this transmit queue.
	///
	/// Used as ?legacy alternative to receive queue affinity.
	///
	/// Default is disabled.
	///
	/// Returns `Ok(true)` if successfully set.
	///
	/// ***Only supported if the network device is multiqueue (ie has more than one transmit queue); if not supported, `Ok(false)`.
	/// Most virtual network devices are not multiqueue.
	/// `Ok(false)` is also returned if the file does not exist.
	#[inline(always)]
	pub fn set_transmit_packet_steering_hyper_thread_affinity(&self, sys_path: &SysPath, hyper_threads: &HyperThreads) -> io::Result<bool>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/xps_cpus");
		
		let file_path = self.xps_cpus_file_path(sys_path);
		
		if file_path.exists()
		{
			let mask = IntoBitMask(hyper_threads);
			match file_path.write_value(mask)
			{
				Ok(()) => Ok(true),
				
				Err(error) => if error.raw_os_error() == Some(ENOENT)
				{
					Ok(false)
				}
				else
				{
					Err(error)
				}
			}
		}
		else
		{
			Ok(false)
		}
	}
	
	/// Transmit Packet Steering (XPS) mapping of receive queues to this transmit queue.
	///
	/// Used as a (better) alternative to HyperThread affinity.
	///
	/// Default is disabled, but, if enabled, the commonest pattern is to have 1 transmit queue mapped to just 1 receive queue (ideally, using the same `QueueIdentifier`).
	#[inline(always)]
	pub fn transmit_packet_steering_receive_queue_affinity(&self, sys_path: &SysPath) -> io::Result<QueueIdentifiers>
	{
		self.xps_rxqs_file_path(sys_path).parse_comma_separated_bit_set().map(QueueIdentifiers)
	}
	
	/// Transmit Packet Steering (XPS) mapping of receive queues to this transmit queue.
	///
	/// Used as a (better) alternative to HyperThread affinity.
	///
	/// Default is disabled, but, if enabled, the commonest pattern is to have 1 transmit queue mapped to just 1 receive queue (ideally, using the same `QueueIdentifier`).
	///
	/// `receive_queue_to_map_to` must not have a size greater than the number of receive queues supported by the network device.
	///
	/// This number can be obtained from:-
	///
	/// * `GetLinkMessageData.number_of_receive_queues` (using Route Netlink)
	/// * or possibly `NetworkDeviceInputOutputControl.number_of_channels().combined / receive_only`; internally in Linux these values, which should be the same, are obtained via different driver functions.
	#[inline(always)]
	pub fn set_transmit_packet_steering_receive_queue_affinity(&self, sys_path: &SysPath, receive_queues_to_map_to: &QueueIdentifiers) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/xps_rxqs");
		
		let file_path = self.xps_rxqs_file_path(sys_path);
		
		if file_path.exists()
		{
			let mask = IntoBitMask(receive_queues_to_map_to);
			file_path.write_value(mask)
		}
		else
		{
			Ok(())
		}
	}
	
	/// Time interval to measure across.
	///
	/// Default is 1,000 milliseconds.
	#[inline(always)]
	pub fn byte_limits_hold_time(&self, sys_path: &SysPath) -> io::Result<Milliseconds>
	{
		self.byte_queue_limits_hold_time_file_path(sys_path).read_value()
	}
	
	/// Time interval to measure across.
	///
	/// Default is 1,000 milliseconds.
	#[inline(always)]
	pub fn set_byte_limits_hold_time(&self, sys_path: &SysPath, hold_time: Milliseconds) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/byte_queue_limits/hold_time");
		
		let file_path = self.byte_queue_limits_hold_time_file_path(sys_path);
		
		if file_path.exists()
		{
			file_path.write_value(hold_time)
		}
		else
		{
			Ok(())
		}
	}
	
	/// Number of bytes currently inflight.
	#[inline(always)]
	pub fn number_of_bytes_inflight(&self, sys_path: &SysPath) -> io::Result<u64>
	{
		self.byte_queue_limits_file_path(sys_path, "inflight").read_value()
	}
	
	/// Minimum value for current limit on bytes to transmit.
	///
	/// Default is 0 bytes.
	#[inline(always)]
	pub fn minimum_current_byte_limit(&self, sys_path: &SysPath) -> io::Result<Option<NonZeroU64>>
	{
		self.byte_queue_limits_limit_min_file_path(sys_path).read_value()
	}
	
	/// Minimum value for current limit on bytes to transmit.
	///
	/// Default is 0 bytes (off).
	#[inline(always)]
	pub fn set_minimum_current_byte_limit(&self, sys_path: &SysPath, minimum_current_byte_limit: Option<NonZeroU64>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/byte_queue_limits/limit_min");
		
		let file_path = self.byte_queue_limits_limit_min_file_path(sys_path);
		
		if file_path.exists()
		{
			let value: u64 = unsafe { transmute(minimum_current_byte_limit) };
			file_path.write_value(UnpaddedDecimalInteger(value))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Current limit on bytes to transmit.
	///
	/// Default is 0 bytes (off).
	#[inline(always)]
	pub fn current_byte_limit(&self, sys_path: &SysPath) -> io::Result<Option<NonZeroU64>>
	{
		self.byte_queue_limits_limit_file_path(sys_path).read_value()
	}
	
	/// Minimum value for current limit on bytes to transmit.
	///
	/// Default is 0 bytes (off).
	#[inline(always)]
	pub fn set_current_byte_limit(&self, sys_path: &SysPath, current_byte_limit: Option<NonZeroU64>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/byte_queue_limits/limit");
		
		let file_path = self.byte_queue_limits_limit_file_path(sys_path);
		
		if file_path.exists()
		{
			let value: u64 = unsafe { transmute(current_byte_limit) };
			file_path.write_value(UnpaddedDecimalInteger(value))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Maximum value for current limit on bytes to transmit.
	///
	/// Default is 1,879,048,192 bytes.
	#[inline(always)]
	pub fn maximum_current_byte_limit(&self, sys_path: &SysPath) -> io::Result<NonZeroU64>
	{
		self.byte_queue_limits_limit_max_file_path(sys_path).read_value()
	}
	
	/// Maximum value for current limit on bytes to transmit.
	///
	/// Default is 0 bytes.
	#[inline(always)]
	pub fn set_maximum_current_byte_limit(&self, sys_path: &SysPath, maximum_current_byte_limit: NonZeroU64) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /sys/class/net/<network_interface_name>/queues/tx-<queue_identifier/byte_queue_limits/limit_max");
		
		let file_path = self.byte_queue_limits_limit_max_file_path(sys_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(maximum_current_byte_limit))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn tx_maxrate_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.file_path(sys_path, "tx_maxrate")
	}
	
	#[inline(always)]
	fn xps_cpus_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.file_path(sys_path, "xps_cpus")
	}
	
	#[inline(always)]
	fn xps_rxqs_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.file_path(sys_path, "xps_rxqs")
	}
	
	#[inline(always)]
	fn byte_queue_limits_limit_min_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.byte_queue_limits_file_path(sys_path, "limit_min")
	}
	
	#[inline(always)]
	fn byte_queue_limits_limit_max_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.byte_queue_limits_file_path(sys_path, "limit_max")
	}
	
	#[inline(always)]
	fn byte_queue_limits_limit_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.byte_queue_limits_file_path(sys_path, "limit")
	}
	
	#[inline(always)]
	fn byte_queue_limits_hold_time_file_path(&self, sys_path: &SysPath) -> PathBuf
	{
		self.byte_queue_limits_file_path(sys_path, "hold_time")
	}
	
	#[inline(always)]
	fn byte_queue_limits_file_path(&self, sys_path: &SysPath, file_name: &str) -> PathBuf
	{
		self.file_path(sys_path, "byte_queue_limits").append(file_name)
	}
}
