// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Congestion control algorithm.
///
/// The Linux default is `cubic` but we use a default of `bbr`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CongestionControlAlgorithm
{
	/// Cubic.
	cubic,

	/// BBR.
	bbr,
}

impl Default for CongestionControlAlgorithm
{
	#[inline(always)]
	fn default() -> Self
	{
		CongestionControlAlgorithm::bbr
	}
}

impl IntoLineFeedTerminatedByteString<'static> for CongestionControlAlgorithm
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'static, [u8]>
	{
		use self::CongestionControlAlgorithm::*;
		
		let bytes = match self
		{
			cubic => b"cubic" as &[u8],
			bbr => b"bbr" as &[u8],
		};
		
		Cow::from(bytes)
	}
}

impl CongestionControlAlgorithm
{
	/// Set value of `/proc/sys/net/ipv4/tcp_congestion_control` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_congestion_control");
		
		let file_path = Self::sys_net_ipv4_tcp_congestion_control_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(self)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_congestion_control_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_ipv4_file_path("tcp_congestion_control")
	}
}
