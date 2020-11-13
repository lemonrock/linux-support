// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Just-in-Time memory allocation limit size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct JustInTimeMemoryAllocationLimitSizeInBytes(pub NonZeroU32);

impl ParseNumber for JustInTimeMemoryAllocationLimitSizeInBytes
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let raw_value = NonZeroU32::parse_number(bytes, radix, parse_byte)?;
		Ok(Self(raw_value))
	}
}

impl Default for JustInTimeMemoryAllocationLimitSizeInBytes
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualGlobalMaximum
	}
}

impl JustInTimeMemoryAllocationLimitSizeInBytes
{
	/// Typical value.
	///
	/// From `/proc/sys/net/core/bpf_jit_limit`.
	pub const UsualGlobalMaximum: Self = Self(new_non_zero_u32(264_241_152));
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/core/bpf_jit_limit`.
	#[inline(always)]
	pub fn global_maximum(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::sys_net_core_bpf_jit_limit_file_path(proc_path).read_value().map(Self)
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/core/bpf_jit_limit` if it exists.
	#[inline(always)]
	pub fn set_global_maximum(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/bpf_jit_limit");
		
		let file_path = Self::sys_net_core_bpf_jit_limit_file_path(proc_path);
		
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
	fn sys_net_core_bpf_jit_limit_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("bpf_jit_limit")
	}
}
