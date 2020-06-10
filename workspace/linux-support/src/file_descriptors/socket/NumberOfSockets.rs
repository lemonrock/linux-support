// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Number of sockets.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NumberOfSockets(NonZeroU32);

impl From<NonZeroU16> for NumberOfSockets
{
	#[inline(always)]
	fn from(value: NonZeroU16) -> Self
	{
		Self::from_u16(value)
	}
}

impl TryFrom<NonZeroU32> for NumberOfSockets
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl NumberOfSockets
{
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(unsafe { NonZeroU32::new_unchecked(1) });
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(unsafe { NonZeroU32::new_unchecked(i32::MAX as u32) });
	
	/// Safe construction.
	#[inline(always)]
	pub const fn from_u16(value: NonZeroU16) -> Self
	{
		Self(unsafe { NonZeroU32::new_unchecked(value.get() as u32) })
	}
	
	/// Typical default.
	///
	/// From `/proc/sys/net/ipv4/tcp_max_orphans`.
	pub const UsualDefaultGlobalMaximumOrphans: Self = Self(unsafe { NonZeroU32::new_unchecked(4096) });
	
	/// Value of `/proc/sys/net/ipv4/tcp_max_orphans`.
	#[inline(always)]
	pub fn global_maximum_orphans(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_max_orphans_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_max_orphans` if it exists.
	#[inline(always)]
	pub fn set_global_maximum_orphans(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_max_orphans");
		
		let file_path = Self::sys_net_ipv4_tcp_max_orphans_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_max_orphans_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("tcp_max_orphans")
	}
	
	/// Typical default.
	///
	/// From `/proc/sys/net/ipv4/tcp_max_tw_buckets`.
	pub const UsualDefaultGlobalMaximumTimeWait: Self = Self(unsafe { NonZeroU32::new_unchecked(4096) });
	
	/// Value of `/proc/sys/net/ipv4/tcp_max_tw_buckets`.
	#[inline(always)]
	pub fn global_maximum_time_wait(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_max_tw_buckets_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_max_tw_buckets` if it exists.
	#[inline(always)]
	pub fn set_global_maximum_time_wait(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_max_tw_buckets");
		
		let file_path = Self::sys_net_ipv4_tcp_max_tw_buckets_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_max_tw_buckets_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("tcp_max_tw_buckets")
	}
}
