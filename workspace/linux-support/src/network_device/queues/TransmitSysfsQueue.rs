// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A transmit queue reference to a folder `/sys/class/net/<network_interface_name>/queues/tx-<N>` where `N` is `queue_identifier`.
///
/// The files `traffic_class` (which is only read only) seems unreadable on my test machine.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
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
			match path.write_value(UnpaddedDecimalInteger(maximum_rate_in_megabits_per_second))
			{
				Ok(()) => Ok(true),
				
				Err(error) => if error.raw_os_error() == EOPNOTSUPP
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
			
			Err(error) => if error.raw_os_error() == ENOENT
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
				
				Err(error) => if error.raw_os_error() == ENOENT
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
}
