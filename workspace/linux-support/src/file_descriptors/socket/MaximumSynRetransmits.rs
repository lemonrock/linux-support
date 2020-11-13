// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Can not exceed `u8::MAX`.
///
/// Number of times to retransmit `SYN`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MaximumSynRetransmits(NonZeroU8);

impl TryFrom<NonZeroU8> for MaximumSynRetransmits
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
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

impl Default for MaximumSynRetransmits
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Default
	}
}

impl MaximumSynRetransmits
{
	/// From `/proc/sys/net/ipv4/tcp_syn_retries`.
	///
	/// Default is 6.
	pub const Default: Self = Self(new_non_zero_u8(6));
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(new_non_zero_u8(1));
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(new_non_zero_u8(i8::MAX as u8));
	
	/// Value of `/proc/sys/net/ipv4/tcp_syn_retries`.
	#[inline(always)]
	pub fn global_maximum(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_syn_retries_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_syn_retries` if it exists.
	#[inline(always)]
	pub fn set_global_maximum(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to `/proc/sys/net/ipv4/tcp_syn_retries`");
		
		let file_path = Self::sys_net_ipv4_tcp_syn_retries_file_path(proc_path);
		
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
	fn sys_net_ipv4_tcp_syn_retries_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_syn_retries")
	}
}
