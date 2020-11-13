// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Not-sent low-water size.
///
/// Set to 16Kb for HTTP/2 prioritization to be effective.
///
/// See <https://lwn.net/Articles/560082/>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NotSentLowWaterInBytes(Option<NonZeroI32>);

impl TryFrom<NonZeroU32> for NotSentLowWaterInBytes
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: NonZeroU32) -> Result<Self, Self::Error>
	{
		let value = value.get();
		if unlikely!(value > Self::InclusiveMaximum.0.unwrap().get() as u32)
		{
			return Err(ParseNumberError::TooLarge)
		}
		Ok(Self(Some(new_non_zero_i32(value as i32))))
	}
}

impl TryFrom<Option<NonZeroU32>> for NotSentLowWaterInBytes
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: Option<NonZeroU32>) -> Result<Self, Self::Error>
	{
		match value
		{
			None => Ok(Self::UsualGlobalDefault),
			Some(value) => Self::try_from(value)
		}
	}
}

impl Into<i32> for NotSentLowWaterInBytes
{
	#[inline(always)]
	fn into(self) -> i32
	{
		match self.0
		{
			None => u32::MAX as i32,
			Some(value) => value.get() as i32,
		}
	}
}

impl ParseNumber for NotSentLowWaterInBytes
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let value = NonZeroU32::parse_number(bytes, radix, parse_byte)?;
		if likely!(value.get() == u32::MAX)
		{
			Ok(Self::UsualGlobalDefault)
		}
		else
		{
			Self::try_from(value)
		}
	}
}

impl Default for NotSentLowWaterInBytes
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualGlobalDefault
	}
}

impl NotSentLowWaterInBytes
{
	/// Is off.
	#[inline(always)]
	pub fn is_off(self) -> bool
	{
		self.0.is_none()
	}
	
	/// Is on.
	#[inline(always)]
	pub fn is_on(self) -> bool
	{
		self.0.is_some()
	}
	
	/// Maximum.
	pub const InclusiveMaximum: Self = Self(Some(new_non_zero_i32(i32::MAX)));
	
	/// Typical value; off.
	///
	/// From `/proc/sys/net/ipv4/tcp_notsent_lowat`.
	pub const UsualGlobalDefault: Self = Self(None);
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Value of `/proc/sys/net/ipv4/tcp_notsent_lowat`.
	#[inline(always)]
	pub fn global_default(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_notsent_lowat_file_path(proc_path).read_value().unwrap())
	}
	
	/// Default is `Self::UsualGlobalDefault`.
	///
	/// Set value of `/proc/sys/net/ipv4/tcp_notsent_lowat` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_notsent_lowat");
		
		let file_path = Self::sys_net_ipv4_tcp_notsent_lowat_file_path(proc_path);
		
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
	fn sys_net_ipv4_tcp_notsent_lowat_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_notsent_lowat")
	}
}
