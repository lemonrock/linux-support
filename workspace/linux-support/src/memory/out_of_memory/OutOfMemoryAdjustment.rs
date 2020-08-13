// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Out of memory adjustment.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum OutOfMemoryAdjustment
{
	/// More likely to be killed.
	MoreLikely(MoreLikelyAdjustment),
	
	/// The default.
	Neither,
	
	/// Less likely to be killed; requires root to change.
	LessLikely(LessLikelyAdjustment),
	
	/// Exempt; requires root to change.
	Exempt,
}

impl Default for OutOfMemoryAdjustment
{
	#[inline(always)]
	fn default() -> Self
	{
		OutOfMemoryAdjustment::Neither
	}
}

impl PartialOrd for OutOfMemoryAdjustment
{
	#[inline(always)]
	fn partial_cmp(&self, right: &Self) -> Option<Ordering>
	{
		Some(self.cmp(right))
	}
}

impl Ord for OutOfMemoryAdjustment
{
	#[inline(always)]
	fn cmp(&self, right: &Self) -> Ordering
	{
		(*self).to_value().cmp(&(*right).to_value())
	}
}

impl OutOfMemoryAdjustment
{
	/// Set value of `/proc/<PID>/oom_adj`.
	#[inline(always)]
	pub fn set(self, proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<()>
	{
		let value = self.to_value();
		if value < 0
		{
			assert_effective_user_id_is_root("write negative value to `/proc/<PID>/oom_adj`");
		}
		
		proc_path.process_file_path(process_identifier, "oom_adj").write_value(UnpaddedDecimalInteger(value))
	}
	
	#[inline(always)]
	fn to_value(self) -> i8
	{
		use self::OutOfMemoryAdjustment::*;
		
		match self
		{
			MoreLikely(adjustment) => adjustment as i8,
			
			Neither => 0,
			
			LessLikely(adjustment) => adjustment as i8,
			
			Exempt => -17,
		}
	}
}
