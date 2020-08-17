// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryEventStatistics
{
	/// The number of times the cgroup is reclaimed due to high memory pressure even though its usage is under the low boundary.
	///
	/// This usually indicates that the low boundary is over-committed.
	pub low: usize,
	
	/// The number of times processes of the cgroup are throttled and routed to perform direct memory reclaim because the high memory boundary was exceeded.
	///
	/// For a cgroup whose memory usage is capped by the high limit rather than global memory pressure, this event’s occurrences are expected.
	pub high: usize,
	
	/// The number of times the cgroup’s memory usage was about to go over the max boundary.
	///
	/// If direct reclaim fails to bring it down, the cgroup goes into an Out-of-Memory (`OOM`) state.
	pub maximum: usize,
	
	/// The number of time the cgroup’s memory usage was reached the limit and allocation was about to fail.
	///
	/// Depending on context the result could be an invocation of the Out-of-Memory (`OOM`) killer and retrying allocation or failing allocation.
	/// Failed allocation in its turn could be returned into userspace as `ENOMEM` or silently ignored in cases like disk read ahead.
	/// For now OOM in memory cgroup kills tasks if shortage has happened inside a page fault.
	pub oom: usize,
	
	/// The number of processes belonging to this cgroup killed by any kind of Out-of-Memory (`OOM`) killer.
	pub oom_kill: usize,
}

impl MemoryEventStatistics
{
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		let mut low = None;
		let mut high = None;
		let mut max = None;
		let mut oom = None;
		let mut oom_kill = None;
		parse_key_value_statistics(file_path, &mut |name, value|
		{
			match name
			{
				b"low" =>
				{
					low = Some(value);
				}
				
				b"high" =>
				{
					high = Some(value);
				}
				
				b"max" =>
				{
					max = Some(value);
				}
				
				b"oom" =>
				{
					oom = Some(value);
				}
				
				b"oom_kill" =>
				{
					oom_kill = Some(value);
				}
				
				_ => (),
			};
			Ok(())
		})?;
		
		Ok
		(
			Self
			{
				low: unwrap_statistic(low, b"low")?,
				high: unwrap_statistic(high, b"high")?,
				maximum: unwrap_statistic(max, b"max")?,
				oom: unwrap_statistic(oom, b"oom")?,
				oom_kill: unwrap_statistic(oom_kill, b"oom_kill")?,
			}
		)
	}
}
