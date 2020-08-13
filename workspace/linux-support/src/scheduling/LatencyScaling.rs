// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Controls whether the scheduler can adjust `latency`.
///
/// The adjustment made is based on the number of CPUs, and increases logarithmically or linearly as implied in the available values.
///
/// This is due to the fact that with more CPUs there is an apparent reduction in perceived latency.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum LatencyScaling
{
	#[allow(missing_docs)]
	DoNotAdjustLatency = 0,
	
	#[allow(missing_docs)]
	LogarithmicAdjustment = 1,
	
	#[allow(missing_docs)]
	LinearAdjustment = 2,
}

impl Default for LatencyScaling
{
	#[inline(always)]
	fn default() -> Self
	{
		LatencyScaling::DoNotAdjustLatency
	}
}

impl LatencyScaling
{
	/// Read.
	#[inline(always)]
	pub fn read(proc_path: &ProcPath) -> io::Result<Self>
	{
		let value: u8 = Self::file_path(proc_path).read_value()?;
		if value <= 2
		{
			Ok(unsafe { transmute(value) })
		}
		else
		{
			Err(io::Error::new(ErrorKind::InvalidData, "Value out of range"))
		}
	}
	
	/// Write.
	#[inline(always)]
	pub fn write(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /proc/sys/kernel/sched_tunable_scaling");
		
		let file_path = Self::file_path(proc_path);
		
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
	fn file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("sched_tunable_scaling")
	}
}
