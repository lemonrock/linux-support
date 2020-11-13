// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 1010 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Retries tunable.
///
/// "This is how many retries it does before it tries to figure out if the gateway is down.
/// Minimal RFC value is 3; it corresponds to ~3sec-8min depending on RTO".
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Retries1(NonZeroU8);

impl TryFrom<NonZeroU8> for Retries1
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error>
	{
		if unlikely!(value < Self::InclusiveMinimum.0)
		{
			return Err(ParseNumberError::TooSmall)
		}
		Ok(Self(value))
	}
}

impl ParseNumber for Retries1
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Self::try_from(NonZeroU8::parse_number(bytes, radix, parse_byte)?)
	}
}

impl Default for Retries1
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualGlobalDefault
	}
}

impl Retries1
{
	/// Minimum.
	pub const InclusiveMinimum: Self = Self(new_non_zero_u8(3));
	
	/// Maximum.
	pub const InclusiveMaximum: Self = Self(new_non_zero_u8(u8::MAX));
	
	/// Default is 3.
	///
	/// From `/proc/sys/net/ipv4/tcp_retries1`.
	pub const UsualGlobalDefault: Self = Self::InclusiveMinimum;
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/ipv4/tcp_retries1`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_retries1_file_path(proc_path).read_value().unwrap())
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/ipv4/tcp_retries1` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_retries1");
		
		let file_path = Self::sys_net_ipv4_tcp_retries1_file_path(proc_path);
		
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
	fn sys_net_ipv4_tcp_retries1_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_retries1")
	}
}
