// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Statistics.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statistics
{
	/// This is the total number of visible (living) descendant cgroups underneath this cgroup.
	///
	/// Known in Linux as `nr_descendants`.
	pub number_of_living_descendants: usize,

	/// This is the total number of dying descendant cgroups underneath this cgroup.
	/// A cgroup enters the dying state after being deleted.
	/// It remains in that state for an undefined period (which will depend on system load) while resources are freed before the cgroup is destroyed.
	/// Note that the presence of some cgroups in the dying state is normal, and is not indicative of any problem.
	///
	/// A process can't be made a member of a dying cgroup, and a dying cgroup can't be brought back to life.
	///
	/// Known in Linux as `nr_dying_descendants`.
	pub number_of_dying_descendants: usize,
}

impl Statistics
{
	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, StatisticsParseError>
	{
		use self::StatisticsParseError::*;

		let mut number_of_living_descendants = None;
		let mut number_of_dying_descendants = None;

		let file = File::open(file_path)?;
		let reader = BufReader::new(file);
		for line in reader.lines()
		{
			let line = line?;
			let mut name_and_value = line.splitn(2, ' ');
			let name = name_and_value.next().expect("Split always should produce at least one item");

			#[inline(always)]
			fn parse_value<'a>(name: &str, mut name_and_value: impl Iterator<Item=&'a str>) -> Result<usize, StatisticsParseError>
			{
				let string_value = name_and_value.next().ok_or(MissingStatisticValue { name: name.to_string() })?;
				let value: Result<usize, ParseIntError> = string_value.parse();
				value.map_err(|cause| InvalidStatisticValue { name: name.to_string(), value: string_value.to_string(), cause })
			}

			match name
			{
				"nr_descendants" =>
				{
					number_of_living_descendants = Some(parse_value(name, name_and_value)?);
				}

				"nr_dying_descendants" =>
				{
					number_of_dying_descendants = Some(parse_value(name, name_and_value)?);
				}

				_ => return Err(InvalidStatisticName { name: name.to_string() }),
			}
		}

		Ok
		(
			Self
			{
				number_of_living_descendants: number_of_living_descendants.ok_or(StatisticNumberOfLivingDescendantsMissing)?,
				number_of_dying_descendants: number_of_dying_descendants.ok_or(StatisticNumberOfDyingDescendantsMissing)?,
			}
		)
	}
}
