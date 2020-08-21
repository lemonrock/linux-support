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

		match self
		{
			&LeaveAsIs => (),

			&DropAllSupplementaryGroups => Groups::drop_all_supplementary_groups(),

			&ChangeTo(ref supplementary_group_choices) =>
			{
				let list = Self::supplementary_group_identifiers(etc_path, supplementary_group_choices)?;
				let length = list.len();
				if unlikely!(length == 0)
				{
					Groups::drop_all_supplementary_groups()
				}
				else
				{
					Groups::set_groups(length, list.as_ptr())
				}
			}
		}
		Ok(())
	}

	#[inline(always)]
	fn supplementary_group_identifiers(etc_path: &EtcPath, supplementary_group_choices: &Vec<SupplementaryGroupChoice>) -> Result<Vec<gid_t>, UserAndGroupChoiceError>
	{
		use self::UserAndGroupChoiceError::*;

		let length = supplementary_group_choices.len();
		if unlikely!(length < Groups::NGROUPS_MAX)
		{
			return Err(TooManySupplementaryGroups(length))
		}

		let mut check = HashSet::with_capacity(length);
		let mut list = Vec::with_capacity(length);
		let iterator = supplementary_group_choices.iter().map(|supplementary_group_choice| supplementary_group_choice.to_group_identifier(etc_path));
		for result in iterator
		{
			let group_identifier = result?;
			if unlikely!(!check.insert(group_identifier))
			{
				return Err(DuplicateGroupIdentifier(group_identifier))
			}
			list.push(group_identifier.0)
		}
		Ok(list)
	}
}
