// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User and group settings.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserAndGroupSettings
{
	/// Used for the real user identifier.
	///
	/// For an unprivileged process (ie one without root or the `CAP_SETUID` and `CAP_SETGID` capabilities), the only valid value is `UserAndGroupChoice::CurrentReal`.
	/// This is the default.
	#[serde(default = "UserAndGroupSettings::real_default")] pub real: UserAndGroupChoice,

	/// Used to set the effective user identifier.
	///
	/// For an unprivileged process (ie one without root or the `CAP_SETUID` and `CAP_SETGID` capabilities), the only valid value is `UserAndGroupChoice::CurrentEffective`.
	/// This is the default.
	#[serde(default = "UserAndGroupSettings::effective_default")] pub effective: UserAndGroupChoice,

	/// Used to set the saved-set user identifier.
	///
	/// For an unprivileged process (ie one without root or the `CAP_SETUID` and `CAP_SETGID` capabilities), the only valid value is `UserAndGroupChoice::CurrentEffective`.
	/// This is the default.
	#[serde(default = "UserAndGroupSettings::saved_set_default")] pub saved_set: UserAndGroupChoice,

	/// Used to set the file system user identifier.
	///
	/// For an unprivileged process (ie one without root or the `CAP_SETUID` and `CAP_SETGID` capabilities), the only valid value is `UserAndGroupChoice::FromCurrentEffective` (sic; not a typo).
	/// This is the default.
	///
	/// This is a deprecated concept but included for completeness.
	#[serde(default = "UserAndGroupSettings::file_system_default")] pub file_system: UserAndGroupChoice,

	/// Supplementary groups.
	///
	/// Default is to not add any supplementary groups.
	#[serde(default)] pub supplementary_groups: SupplementaryGroupSetting,
}

impl Default for UserAndGroupSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			real: Self::real_default(),
			effective: Self::effective_default(),
			saved_set: Self::saved_set_default(),
			file_system: Self::file_system_default(),
			supplementary_groups: SupplementaryGroupSetting::default(),
		}
	}
}

impl UserAndGroupSettings
{
	/// * Sets all 4 user identifiers and all 4 group identifiers.
	/// * Initializes groups.
	///
	/// Note: Can end up parsing `/etc/passwd` 4 times.
	#[inline(always)]
	pub fn change_user_and_groups(&self, etc_path: &EtcPath) -> Result<(), UserAndGroupChoiceError>
	{
		let (real_user_identifier, real_group_identifier) = self.real.to_user_and_group_identifiers(etc_path)?;
		let (effective_user_identifier, effective_group_identifier) = self.effective.to_user_and_group_identifiers(etc_path)?;
		let (saved_set_user_identifier, saved_set_group_identifier) = self.saved_set.to_user_and_group_identifiers(etc_path)?;
		let (file_system_user_identifier, file_system_group_identifier) = self.file_system.to_user_and_group_identifiers(etc_path)?;

		UserIdentifier::set_real_effective_and_saved_set(Some(real_user_identifier), Some(effective_user_identifier), Some(saved_set_user_identifier));
		file_system_user_identifier.set_file_system();

		GroupIdentifier::set_real_effective_and_saved_set(Some(real_group_identifier), Some(effective_group_identifier), Some(saved_set_group_identifier));
		file_system_group_identifier.set_file_system();

		self.supplementary_groups.change(etc_path)
	}

	#[inline(always)]
	const fn real_default() -> UserAndGroupChoice
	{
		UserAndGroupChoice::FromCurrentReal
	}

	#[inline(always)]
	const fn effective_default() -> UserAndGroupChoice
	{
		UserAndGroupChoice::FromCurrentEffective
	}

	#[inline(always)]
	const fn saved_set_default() -> UserAndGroupChoice
	{
		UserAndGroupChoice::FromCurrentSavedSet
	}

	/// Yes, this is meant to return `FromCurrentEffective`.
	#[inline(always)]
	const fn file_system_default() -> UserAndGroupChoice
	{
		UserAndGroupChoice::FromCurrentEffective
	}
}
