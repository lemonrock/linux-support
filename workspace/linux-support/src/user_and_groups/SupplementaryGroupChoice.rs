// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Supplementary group choice.
///
/// `FromCurrentFileSystem` is not a member because it is impossible without parsing `/proc/self/status` to find the current file system group identifier.
/// Additionally, the ony valid value for an unprivileged process (ie one without root and the `CAP_SETGID` capabilities) is actually `FromCurrentEffective`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SupplementaryGroupChoice
{
	/// Sets the supplementary group by looking up the group name in `/etc/group`.
	FromGroupNameInEtcGroup(GroupName),

	/// Specific value that does not require `/etc/group` to exist.
	FromGroupIdentifier(GroupIdentifier),

	/// From current real group.
	FromCurrentReal,

	/// From current effective group.
	FromCurrentEffective,

	/// From current saved-set group.
	FromCurrentSavedSet,
}

impl SupplementaryGroupChoice
{
	/// To group identifier.
	#[inline(always)]
	pub fn to_group_identifier(&self, etc_path: &EtcPath) -> Result<GroupIdentifier, UserAndGroupChoiceError>
	{
		use self::SupplementaryGroupChoice::*;

		Ok
		(
			match self
			{
				&FromGroupNameInEtcGroup(ref group_name) => UserAndGroupChoice::etc_group_identifier_for_group_name(etc_path, group_name)?,

				&FromGroupIdentifier(group_identifier) => group_identifier,

				&FromCurrentReal => GroupIdentifier::current_real(),

				&FromCurrentReal => GroupIdentifier::current_effective(),

				&FromCurrentReal => GroupIdentifier::current_real_effective_and_saved_set().2,
			}
		)
	}
}
