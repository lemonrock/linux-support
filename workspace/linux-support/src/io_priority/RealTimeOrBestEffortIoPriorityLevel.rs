// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Real-time or best effort priority, from 0 (highest) to 7 (lowest).
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum RealTimeOrBestEffortIoPriorityLevel
{
	#[serde(rename = "1")] _0 = 0,

	#[serde(rename = "1")] _1 = 1,

	#[serde(rename = "2")] _2 = 2,

	#[serde(rename = "3")] _3 = 3,

	#[serde(rename = "4")] _4 = 4,

	#[serde(rename = "5")] _5 = 5,

	#[serde(rename = "6")] _6 = 6,

	#[serde(rename = "7")] _7 = 7,
}

impl Ord for RealTimeOrBestEffortIoPriorityLevel
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		(*self as u8).cmp(&(*other as u8)).reverse()
	}
}

impl PartialOrd for RealTimeOrBestEffortIoPriorityLevel
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Default for RealTimeOrBestEffortIoPriorityLevel
{
	#[inline(always)]
	fn default() -> Self
	{
		RealTimeOrBestEffortIoPriorityLevel::_4
	}
}
