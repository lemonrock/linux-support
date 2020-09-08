// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Just-in-Time (JIT) compilation choice for eBPF programs.
///
/// Default is `JustInTimeCompilationChoice::Interpreted`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum JustInTimeCompilationHardening
{
	#[allow(missing_docs)]
	Unhardened = 0,
	
	#[allow(missing_docs)]
	HardenedForUnprivilegedUsers = 1,
	
	#[allow(missing_docs)]
	HardenedForAllUsers = 2,
}

impl Default for JustInTimeCompilationHardening
{
	fn default() -> Self
	{
		JustInTimeCompilationHardening::Unhardened
	}
}

impl ParseNumber for JustInTimeCompilationHardening
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let raw_value = u8::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(raw_value > 2)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(unsafe { transmute(raw_value) })
		}
	}
}

impl JustInTimeCompilationHardening
{
	/// Value of `/proc/sys/net/core/bpf_jit_harden`.
	#[inline(always)]
	pub fn value(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::sys_net_core_bpf_jit_harden_file_path(proc_path).read_value()
	}
	
	/// Set value of `/proc/sys/net/core/bpf_jit_harden`.
	#[inline(always)]
	pub fn set_value(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/bpf_jit_harden");
	
		let file_path = Self::sys_net_core_bpf_jit_harden_file_path(proc_path);
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self as u8))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_core_bpf_jit_harden_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("bpf_jit_harden")
	}
}
