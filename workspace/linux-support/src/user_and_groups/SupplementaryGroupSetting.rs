// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Change supplementary groups.
///
/// Default is `LeaveAsIs`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SupplementaryGroupSetting
{
	#[allow(missing_docs)]
	LeaveAsIs,

	#[allow(missing_docs)]
	DropAllSupplementaryGroups,

	#[allow(missing_docs)]
	ChangeTo(Vec<SupplementaryGroupChoice>),
}

impl Default for SupplementaryGroupSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		SupplementaryGroupSetting::LeaveAsIs
	}
}

impl SupplementaryGroupSetting
{
	/// Change.
	///
	/// Requires the capability `CAP_SETGID`.
	pub fn change(&self, etc_path: &EtcPath) -> Result<(), UserAndGroupChoiceError>
	{
		use self::SupplementaryGroupSetting::*;

		let result = match self
		{
			&LeaveAsIs => return Ok(()),

			&DropAllSupplementaryGroups => Self::drop_all_supplementary_groups(),

			&ChangeTo(ref supplementary_group_choices) =>
			{
				let list = Self::supplementary_group_identifiers(etc_path, supplementary_group_choices)?;
				let length = list.length();
				let pointer = if length == 0
				{
					Self::drop_all_supplementary_groups()
				}
				else
				{
					unsafe { setgroups(length, list.as_ptr()) }
				};
			}
		};

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("list has an invalid address, or size is greater than NGROUPS_MAX (32 before Linux 2.6.4; 65536 since Linux 2.6.4)"),
				ENOMEM => panic!("Out of memory"),
				EPERM => panic!("Permission denied"),

				unexpected @ _ => panic!("Unexpected error {} from setgroups()", unexpected)
			}
		}
		else
		{
			unreachable!("setgroups() returned an unexpected result of {}", result)
		}
	}

	#[inline(always)]
	fn drop_all_supplementary_groups() -> i32
	{
		unsafe { setgroups(0, null()) }
	}

	#[inline(always)]
	fn supplementary_group_identifiers(etc_path: &EtcPath, supplementary_group_choices: &Vec<SupplementaryGroupChoice>) -> Result<Vec<gid_t>, UserAndGroupChoiceError>
	{
		use self::UserAndGroupChoiceError::*;

		let length = supplementary_group_choices.len();
		// Was 32 before Linux 2.6.4.
		const NGROUPS_MAX: usize = 65536;
		if unlikely!(length >= NGROUPS_MAX)
		{
			return Err(TooManySupplementaryGroups(length))
		}

		let mut check = HashSet::with_capacity(length);
		let mut list = Vec::with_capacity(length);
		let iterator = supplementary_group_choices.iter().map(|supplementary_group_choice| supplementary_group_choice.to_group_identifier(etc_path).0);
		for result in iterator
		{
			let group_identifier = result?;
			if unlikely(!check.insert(group_identifier))
			{
				return Err(DuplicateGroupIdentifier(group_identifier))
			}
			list.push(group_identifier)
		}
		Ok(list)
	}
}
