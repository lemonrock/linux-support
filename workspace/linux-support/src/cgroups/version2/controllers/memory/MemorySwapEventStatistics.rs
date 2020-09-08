// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemorySwapEventStatistics
{
	/// The number of times the cgroup’s swap usage was about to go over the max boundary and swap allocation failed.
	pub maximum: usize,
	
	/// The number of times swap allocation failed either because of running out of swap system-wide or max limit.
	pub fail: usize,
}

impl MemorySwapEventStatistics
{
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		let mut max = None;
		let mut fail = None;
		parse_key_value_statistics(file_path, &mut |name, value|
		{
			match name
			{
				b"max" =>
				{
					max = Some(value);
				}
				
				b"fail" =>
				{
					fail = Some(value);
				}
				
				_ => (),
			};
			Ok(())
		})?;
		
		Ok
		(
			Self
			{
				maximum: unwrap_statistic(max, b"max")?,
				fail: unwrap_statistic(fail, b"fail")?,
			}
		)
	}
}
