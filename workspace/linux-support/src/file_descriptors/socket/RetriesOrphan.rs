// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Retries tunable.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum RetriesOrphan
{
	/// Does 8 retries with a special case; see `tcp_orphan_retries()` in the Linux kernel sources.
	Special,

	/// Number of retries.
	Retries(NonZeroU8),
}

impl ParseNumber for RetriesOrphan
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		use self::RetriesOrphan::*;
		let value = u8::parse_number(bytes, radix, parse_byte)?;
		Ok
		(
			if likely!(value == 0)
			{
				Special
			}
			else
			{
				Retries(new_non_zero_u8(value))
			}
		)
	}
}

impl Default for RetriesOrphan
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualGlobalDefault
	}
}

impl RetriesOrphan
{
	/// Default is 0.
	///
	/// From `/proc/sys/net/ipv4/tcp_orphan_retries`.
	pub const UsualGlobalDefault: Self = RetriesOrphan::Special;
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/ipv4/tcp_orphan_retries`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self::sys_net_ipv4_tcp_orphan_retries_file_path(proc_path).read_value().unwrap()
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/ipv4/tcp_orphan_retries` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_orphan_retries");
		
		let file_path = Self::sys_net_ipv4_tcp_orphan_retries_file_path(proc_path);
		
		if file_path.exists()
		{
			use self::RetriesOrphan::*;
			
			let value = match self
			{
				Special => 0u8,
				Retries(value) => value.get(),
			};
			file_path.write_value(UnpaddedDecimalInteger(value))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_orphan_retries_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_orphan_retries")
	}
}
