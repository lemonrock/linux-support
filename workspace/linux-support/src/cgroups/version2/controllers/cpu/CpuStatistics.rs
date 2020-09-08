// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU statistics.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CpuStatistics
{
	#[allow(missing_docs)]
	pub usage_time: Microseconds,
	
	#[allow(missing_docs)]
	pub user_time: Microseconds,
	
	#[allow(missing_docs)]
	pub system_time: Microseconds,
	
	#[allow(missing_docs)]
	pub only_when_controller_enabled: Option<OnlyWhenControllerEnabledCpuStatistics>,
}

impl CpuStatistics
{
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		use self::StatisticsParseError::*;
		
		let mut usage_usec = None;
		let mut user_usec = None;
		let mut system_usec = None;
		let mut nr_periods = None;
		let mut nr_throttled = None;
		let mut throttled_usec = None;
		parse_key_value_statistics(file_path, &mut |name, value| match name
		{
			b"usage_usec" =>
			{
				usage_usec = Some(parse_usize_to_microseconds(name, value)?);
				Ok(())
			}
			
			b"user_usec" =>
			{
				user_usec = Some(parse_usize_to_microseconds(name, value)?);
				Ok(())
			}
			
			b"system_usec" =>
			{
				system_usec = Some(parse_usize_to_microseconds(name, value)?);
				Ok(())
			}
			
			b"nr_periods" =>
			{
				nr_periods = Some(value);
				Ok(())
			}
			
			b"nr_throttled" =>
			{
				nr_throttled = Some(value);
				Ok(())
			}
			
			b"throttled_usec" =>
			{
				throttled_usec = Some(parse_usize_to_microseconds(name, value)?);
				Ok(())
			}
			
			_ => Ok(()),
		})?;
		
		let only_when_controller_enabled = match (nr_periods, nr_throttled, throttled_usec)
		{
			(None, None, None) => None,
			
			(Some(nr_periods), Some(nr_throttled), Some(throttled_usec)) => Some
			(
				OnlyWhenControllerEnabledCpuStatistics
				{
					number_of_periods: nr_periods,
					number_throttled: nr_throttled,
					throttled_time: throttled_usec,
				}
			),
			
			_ => return Err(MissingOneOrMoreStatistics)
		};
		
		Ok
		(
			Self
			{
				usage_time: unwrap_statistic(usage_usec, b"usage_usec")?,
				user_time: unwrap_statistic(user_usec, b"user_usec")?,
				system_time: unwrap_statistic(system_usec, b"system_usec")?,
				only_when_controller_enabled,
			}
		)
	}
}
