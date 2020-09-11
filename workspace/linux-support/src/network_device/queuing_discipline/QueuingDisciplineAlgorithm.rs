// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Congestion control algorithm.
///
/// The Linux default is `pfifo_fast` but we use a default of `fq`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum QueuingDisciplineAlgorithm
{
	/// pfifo_fast.
	pfifo_fast,

	/// fq.
	fq,
	
	/// Used for devices such as `lo` and `veth`.
	noqueue,
}

impl Default for QueuingDisciplineAlgorithm
{
	#[inline(always)]
	fn default() -> Self
	{
		QueuingDisciplineAlgorithm::fq
	}
}

impl IntoLineFeedTerminatedByteString<'static> for QueuingDisciplineAlgorithm
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'static, [u8]>
	{
		use self::QueuingDisciplineAlgorithm::*;
		
		let bytes = match self
		{
			pfifo_fast => b"pfifo_fast" as &[u8],
			fq => b"fq" as &[u8],
			noqueue => b"noqueue" as &[u8],
		};
		
		Cow::from(bytes)
	}
}

impl QueuingDisciplineAlgorithm
{
	/// Set value of `/proc/sys/net/core/default_qdisc` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/default_qdisc");
		
		let file_path = Self::sys_net_core_default_qdisc_file_path(proc_path);
		
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
	fn sys_net_core_default_qdisc_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("default_qdisc")
	}
}
