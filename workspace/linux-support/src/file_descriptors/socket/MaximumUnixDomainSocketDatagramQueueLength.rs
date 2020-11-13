// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ?Can not exceed `u16::MAX`?
///
/// Maximum number of datagrams that can be queued for a Unix Domain Socket.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumUnixDomainSocketDatagramQueueLength(NonZeroU16);

impl Default for MaximumUnixDomainSocketDatagramQueueLength
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl MaximumUnixDomainSocketDatagramQueueLength
{
	/// From `/proc/sys/net/unix/max_dgram_qlen`.
	pub const Default: Self = Self(new_non_zero_u16(10));
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(new_non_zero_u16(1));
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(new_non_zero_u16(u16::MAX));
	
	/// Value of `/proc/sys/net/ipv4/max_dgram_qlen`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_unix_max_dgram_qlen_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/max_dgram_qlen` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/net/ipv4/max_dgram_qlen`");
		
		let file_path = Self::sys_net_unix_max_dgram_qlen_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0.get()))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_unix_max_dgram_qlen_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_unix_file_path("max_dgram_qlen")
	}
}
