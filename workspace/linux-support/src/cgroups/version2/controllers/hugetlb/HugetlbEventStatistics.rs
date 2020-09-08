// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HugetlbEventStatistics
{
	/// The number of allocation failure due to HugeTLB limit.
	pub number_of_allocation_failures_due_to_the_maximum_limit: usize,
}

impl HugetlbEventStatistics
{
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		let mut max = None;
		parse_key_value_statistics(file_path, &mut |name, value|
		{
			match name
			{
				b"max" =>
				{
					max = Some(value);
				}
				
				_ => (),
			};
			Ok(())
		})?;
		
		Ok
		(
			Self
			{
				number_of_allocation_failures_due_to_the_maximum_limit: unwrap_statistic(max, b"max")?,
			}
		)
	}
}
