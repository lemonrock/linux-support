// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventStatistics
{
	/// Are there processes or threads in this cgroup?
	pub populated: bool,

	/// Is this cgroup frozen?
	pub frozen: bool,
}

impl EventStatistics
{
	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		use self::StatisticsParseError::*;
		
		let mut populated = None;
		let mut frozen = None;
		parse_key_value_statistics(file_path, &mut |name, value| match name
		{
			b"populated" =>
			{
				populated = Some(value);
				false
			}
			
			b"frozen" =>
			{
				frozen = Some(value);
				false
			}
			
			_ => true,
		})?;

		Ok
		(
			Self
			{
				populated: populated.ok_or(MissingStatistic(b"populated"))?,
				frozen: frozen.ok_or(MissingStatistic(b"frozen"))?,
			}
		)
	}
}
