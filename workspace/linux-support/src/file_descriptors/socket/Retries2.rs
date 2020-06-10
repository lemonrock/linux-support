// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Retries tunable.
///
/// See <https://pracucci.com/linux-tcp-rto-min-max-and-tcp-retries2.html>.
///
/// This value influences the timeout of an alive TCP connection, when RTO retransmissions remain unacknowledged.
/// Given a value of N, a hypothetical TCP connection following exponential backoff with an initial RTO of TCP_RTO_MIN would retransmit N times before killing the connection at the (N+1)th RTO.
///
/// The default value of 15 yields a hypothetical timeout of 924.6 seconds and is a lower bound for the effective timeout.
/// TCP will effectively time out at the first RTO which exceeds the hypothetical timeout.
///
/// RFC 1122 recommends at least 100 seconds for the timeout, which corresponds to a value of at least 8.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Retries2(NonZeroU8);

impl ParseNumber for Retries2
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		Ok(Self(NonZeroU8::parse_number(bytes, radix, parse_byte)?))
	}
}

impl Default for Retries2
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualGlobalDefault
	}
}

impl Retries2
{
	/// Maximum.
	pub const InclusiveMaximum: Self = Self(unsafe { NonZeroU8::new_unchecked(u8::MAX) });
	
	/// Default is 15.
	///
	/// From `/proc/sys/net/ipv4/tcp_retries2`.
	pub const UsualGlobalDefault: Self = Self(unsafe { NonZeroU8::new_unchecked(15) });
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/ipv4/tcp_retries2`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_retries2_file_path(proc_path).read_value().unwrap())
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/ipv4/tcp_retries2` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_retries2");
		
		let file_path = Self::sys_net_ipv4_tcp_retries2_file_path(proc_path);
		
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
	fn sys_net_ipv4_tcp_retries2_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_retries2")
	}
}
