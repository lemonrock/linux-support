// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Out of memory score adjustment, from -1000 to 1000.
///
/// A value of -1000 (`OutOfMemoryScoreAdjustment::LessLikely(OutOfMemoryScoreAdjustmentValue(InclusiveMaximum))`) will prevent a process being killed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum OutOfMemoryScoreAdjustment
{
	/// More likely to be killed.
	MoreLikely(OutOfMemoryScoreAdjustmentValue),
	
	/// The default.
	Neither,
	
	/// Less likely to be killed; requires root to change.
	LessLikely(OutOfMemoryScoreAdjustmentValue),
}

impl Default for OutOfMemoryScoreAdjustment
{
	#[inline(always)]
	fn default() -> Self
	{
		OutOfMemoryScoreAdjustment::Neither
	}
}

impl PartialOrd for OutOfMemoryScoreAdjustment
{
	#[inline(always)]
	fn partial_cmp(&self, right: &Self) -> Option<Ordering>
	{
		Some(self.cmp(right))
	}
}

impl Ord for OutOfMemoryScoreAdjustment
{
	#[inline(always)]
	fn cmp(&self, right: &Self) -> Ordering
	{
		use self::OutOfMemoryScoreAdjustment::*;
		
		match (self, right)
		{
			(&MoreLikely(ref left_adjustment), &MoreLikely(ref right_adjustment)) => left_adjustment.cmp(right_adjustment),
			
			(&MoreLikely(_), Neither) => Ordering::Greater,
			
			(&MoreLikely(_), LessLikely(_)) => Ordering::Greater,
			
			(&Neither, MoreLikely(_)) => Ordering::Less,
			
			(&Neither, &Neither) => Ordering::Equal,
			
			(&Neither, &LessLikely(_)) => Ordering::Greater,
			
			(&LessLikely(_), &MoreLikely(_)) => Ordering::Less,
			
			(&LessLikely(_), &Neither) => Ordering::Less,
			
			(&LessLikely(ref left_adjustment), &LessLikely(ref right_adjustment)) => right_adjustment.cmp(left_adjustment),
		}
	}
}

impl OutOfMemoryScoreAdjustment
{
	/// Set value of `/proc/<PID>/oom_score_adj`.
	#[inline(always)]
	pub fn set(self, proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<()>
	{
		let value = self.to_value();
		
		let value = self.to_value();
		if value < 0
		{
			assert_effective_user_id_is_root("write negative value to `/proc/<PID>/oom_score_adj`");
		}
		
		proc_path.process_file_path(process_identifier, "oom_score_adj").write_value(UnpaddedDecimalInteger(value))
	}
	
	#[inline(always)]
	fn to_value(self) -> i16
	{
		use self::OutOfMemoryScoreAdjustment::*;
		
		match self
		{
			MoreLikely(adjustment) =>
			{
				let value: i16 = adjustment.into();
				value
			},
			
			Neither => 0,
			
			LessLikely(adjustment) => -(adjustment.into()),
		}
	}
}
