// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User and group settings.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct UserAndGroupSettings
{
	/// Used for the user identifier and home folder path.
	/// Must exist in `/etc/passwd`.
	#[serde(default = "UserAndGroupSettings::user_name_default")] pub user_name: CString,

	/// Used to set the effective user identifier.
	/// Must exist in `/etc/passwd`.
	#[serde(default = "UserAndGroupSettings::user_name_default")] pub effective_user_name: CString,

	/// Used to set the saved-set user identifier.
	/// Must exist in `/etc/passwd`.
	#[serde(default = "UserAndGroupSettings::user_name_default")] pub saved_set_user_name: CString,

	/// Used to set the file system user identifier.
	/// Must exist in `/etc/passwd`.
	/// A deprecated concept but included for completeness.
	#[serde(default = "UserAndGroupSettings::user_name_default")] pub file_system_user_name: CString,

	/// Initialize additional groups.
	/// Requires `/etc/passwd` to exist and for namespace permissions to have been granted.
	#[serde(default = "UserAndGroupSettings::initialize_groups_default")] pub initialize_groups: bool,
}

impl Default for UserAndGroupSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			user_name: Self::user_name_default(),
			effective_user_name: Self::user_name_default(),
			saved_set_user_name: Self::user_name_default(),
			file_system_user_name: Self::user_name_default(),
			initialize_groups: Self::initialize_groups_default(),
		}
	}
}

impl UserAndGroupSettings
{
	/// * Sets all 4 user identifiers and all 4 group identifiers.
	/// * Initializes groups.
	/// * Overwrites the `USER`, `LOGNAME` and `HOME` environment variables.
	///
	/// Requires the capabilities `CAP_SETUID` and `CAP_SETGID`.
	/// Requires `/etc/passwd` to exist.
	/// `initialize_groups`, if `true`, requires `/etc/group` to exist and will fail in most user namespaces.
	#[inline(always)]
	pub fn switch_user(&self, daemonize: Option<&impl OriginalRealUserAndGroupIdentifierUser>) -> (&CStr, CString)
	{
		let (real_user_identifier, real_group_identifier, _) = self.get_and_parse_user_entry(&self.user_name);

		let (effective_user_identifier, effective_group_identifier, effective_user_home_folder_path) = self.get_and_parse_user_entry(&self.effective_user_name);
		// NOTE: The pointer returned from `getpwnam()` is only good until another call to it or a similar method is made!
		let effective_user_home_folder_path = (unsafe { CStr::from_ptr(effective_user_home_folder_path.as_ptr()) }).into();

		let (saved_set_user_identifier, saved_set_group_identifier, _) = self.get_and_parse_user_entry(&self.saved_set_user_name);

		let (file_system_user_identifier, file_system_group_identifier, _) = self.get_and_parse_user_entry(&self.file_system_user_name);

		daemonize.map(|daemonize| daemonize.create_pid_file_before_switching_user_and_group(effective_user_identifier, effective_group_identifier));

		setresuid_wrapper(real_user_identifier, effective_user_identifier, saved_set_user_identifier);
		setresgid_wrapper(real_group_identifier, effective_group_identifier, saved_set_group_identifier);
		unsafe { setfsuid(file_system_user_identifier.into()) };
		unsafe { setfsgid(file_system_group_identifier.into()) };

		if unlikely!(self.initialize_groups)
		{
			initgroups_wrapper(&self.effective_user_name, effective_group_identifier)
		}

		(&self.effective_user_name, effective_user_home_folder_path)
	}

	#[inline(always)]
	fn get_and_parse_user_entry(&self, user_name: &CStr) -> (UserIdentifier, GroupIdentifier, NonNull<c_char>)
	{
		let entry = self.get_user_entry(user_name);
		let entry = unsafe { entry.as_ref() };
		(
			UserIdentifier::from(entry.pw_uid),
			GroupIdentifier::from(entry.pw_gid),
			NonNull::new(entry.pw_dir).expect("pw_dir was null"),
		)
	}

	#[inline(always)]
	fn get_user_entry(&self, user_name: &CStr) -> NonNull<passwd>
	{
		let entry = unsafe { getpwnam(user_name.as_ptr()) };
		assert!(!entry.is_null(), "user name '{:?} does not exist in /etc/passwd", &self.user_name);
		unsafe { NonNull::new_unchecked(entry) }
	}

	#[inline(always)]
	fn user_name_default() -> CString
	{
		CString::new("root").unwrap()
	}

	#[inline(always)]
	fn initialize_groups_default() -> bool
	{
		false
	}
}
