// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How often to do adaptive coalescing packet rate sampling.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct AdaptiveCoalescingRateSampling
{
	/// How often to do adaptive coalescing packet rate sampling, measured in seconds.
	///
	/// Driver must support `ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL` if `Some`.
	pub interval_in_seconds: Option<NonZeroU32>,
}

impl Default for AdaptiveCoalescingRateSampling
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::One
	}
}

impl AdaptiveCoalescingRateSampling
{
	const One: Self = Self
	{
		interval_in_seconds: Some(new_non_zero_u32(1)),
	};
	
	#[inline(always)]
	pub(crate) const fn new_from_ethtool(command: &ethtool_coalesce) -> Self
	{
		Self
		{
			interval_in_seconds: command.rate_sample_interval,
		}
	}
}
