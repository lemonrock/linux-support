// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can not exceed `i32::MAX`.
///
/// The approximate time in microseconds to busy poll on a blocking receive when there is no data.
///
/// Usage requires the capability `CAP_NET_ADMIN`.
///
/// Only exists if the Linux kernel has been configured with `CONFIG_NET_RX_BUSY_POLL`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum BusyPollMicroseconds
{
	/// Off.
	Off,

	/// On.
	On(NonZeroU32),
}

impl Default for BusyPollMicroseconds
{
	#[inline(always)]
	fn default() -> Self
	{
		BusyPollMicroseconds::Off
	}
}

impl BusyPollMicroseconds
{
	/// From `/proc/sys/net/core/busy_read`.
	pub const Default: Self = BusyPollMicroseconds::Off;
	
	/// Value of `/proc/sys/net/core/busy_read`.
	///
	/// This sets the default value of the `SO_BUSY_POLL` socket option.
	///
	/// Low latency busy poll timeout for socket reads (needs `CONFIG_NET_RX_BUSY_POLL`).
	/// Approximate time in microseconds to busy loop waiting for packets on the device queue.
	///
	/// Can be set or overridden per socket by setting socket option `SO_BUSY_POLL`, which is the preferred method of enabling.
	/// If you need to enable the feature globally a value of 50 is recommended.
	///
	/// Will increase power usage.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self::from_file_path(Self::sys_net_core_busy_read_file_path(proc_path))
	}
	
	/// Set value of `/proc/sys/net/core/busy_read` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/net/core/busy_read`");
		
		let file_path = Self::sys_net_core_busy_read_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(self.to_unpadded_decimal_integer())
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_core_busy_read_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("busy_read")
	}
	
	/// From `/proc/sys/net/core/busy_poll`.
	pub const DefaultSelectAndPoll: Self = BusyPollMicroseconds::Off;
	
	/// Value of `/proc/sys/net/core/busy_poll`.
	///
	/// Low latency busy poll timeout for poll and select (needs `CONFIG_NET_RX_BUSY_POLL`).
	/// Approximate time in us to busy loop waiting for events.
	///
	/// Recommended value depends on the number of sockets you poll on; for several sockets 50, for several hundreds 100.
	/// For more than that you probably want to use epoll.
	///
	/// Note that only sockets with `SO_BUSY_POLL` set will be busy polled, so you want to either selectively set the `SO_BUSY_POLL` option on those sockets or change `/proc/sys/net/core/busy_read` to a non-zero amount.
	///
	/// Will increase power usage.
	#[inline(always)]
	pub fn global_select_and_poll_default(proc_path: &ProcPath) -> Self
	{
		Self::from_file_path(Self::sys_net_core_busy_poll_file_path(proc_path))
	}
	
	/// Set value of `/proc/sys/net/core/busy_poll` if it exists.
	#[inline(always)]
	pub fn set_global_select_and_poll_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/net/core/busy_poll`");
		
		let file_path = Self::sys_net_core_busy_poll_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(self.to_unpadded_decimal_integer())
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_core_busy_poll_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("busy_poll")
	}
	
	#[inline(always)]
	fn from_file_path(file_path: PathBuf) -> Self
	{
		let value: u32 = file_path.read_value().unwrap();
		unsafe { transmute(value) }
	}
	
	#[inline(always)]
	fn to_unpadded_decimal_integer(self) -> UnpaddedDecimalInteger<u32>
	{
		unsafe { transmute(self) }
	}
	
	#[inline(always)]
	pub(crate) fn to_socket_option_value(self) -> i32
	{
		unsafe { transmute(self) }
	}
}
