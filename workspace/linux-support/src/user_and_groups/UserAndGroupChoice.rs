// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User and group choice.
///
/// `FromCurrentFileSystem` is not a member because it is impossible without parsing `/proc/self/status` to find the current file system user and group identifiers.
/// Additionally, the ony valid value for an unprivileged process (ie one without root or the `CAP_SETUID` and `CAP_SETGID` capabilities) is actually `FromCurrentEffective`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum UserAndGroupChoice
{
	/// Sets the user and group by looking up the user name in `/etc/passwd`.
	FromUserNameInEtcPasswd(UserName),

	/// Looks up the user name in `/etc/passwd` to find the user identifier.
	///
	/// Looks up the group name in `/etc/group` to find the group identifier.
	FromUserNameInEtcPasswdAndGroupNameInEtcGroup(UserName, GroupName),

	/// Looks up the user identifier in `/etc/passwd` to find the group identifier.
	FromUserIdentifierInEtcPasswd(UserIdentifier),

	/// Specific values that do not require `/etc/passwd` or `/etc/group` to exist.
	FromUserIdentifierAndGroupIdentifier(UserIdentifier, GroupIdentifier),

	/// From current real user and group.
	FromCurrentReal,

	/// From current effective user and group.
	FromCurrentEffective,

	/// From current saved-set user and group.
	FromCurrentSavedSet,
}

impl UserAndGroupChoice
{
	/// Converts.
	#[inline(always)]
	pub fn to_user_and_group_identifiers(&self, etc_path: &EtcPath) -> Result<(UserIdentifier, GroupIdentifier), UserAndGroupChoiceError>
	{
		use self::UserAndGroupChoice::*;
		use self::UserAndGroupChoiceError::*;
		match self
		{
			&FromUserNameInEtcPasswd(ref user_name) => Self::etc_passwd_record_for_user_name(etc_path, user_name),

			&FromUserNameInEtcPasswdAndGroupNameInEtcGroup(ref user_name, ref group_name) =>
			{
				let user_identifier = Self::etc_passwd_record_for_user_name(etc_path, user_name)?.0;
				let group_identifier = Self::etc_group_identifier_for_group_name(etc_path, group_name)?;
				Ok((user_identifier, group_identifier))
			}

			&FromUserIdentifierInEtcPasswd(user_identifier) =>
			{
				Self::etc_passwd_record_for_user_identifier::<(UserIdentifier, GroupIdentifier), UserAndGroupChoiceError>(etc_path, user_identifier, |etc_passwd_record| Ok((etc_passwd_record.user_identifier, etc_passwd_record.group_identifier)))
			}

			&FromUserIdentifierAndGroupIdentifier(user_identifier, group_identifier) => Ok((user_identifier, group_identifier)),

			&FromCurrentReal => (UserIdentifier::current_real(), GroupIdentifier::current_real()),

			&FromCurrentEffective => (UserIdentifier::current_effective(), GroupIdentifier::current_effective()),

			&FromCurrentEffective => (UserIdentifier::current_effective(), GroupIdentifier::current_effective()),
		}
	}

	#[inline(always)]
	fn etc_passwd_record_for_user_identifier<A, B, F: FnOnce(EtcPasswdRecord) -> Result<A, B>>(etc_path: &EtcPath, user_identifier: UserIdentifier, etc_passwd_record_user: F) -> Result<A, B>
	{
		let etc_passwd = EtcPasswd::open(etc_path)?;
		let etc_passwd_record = Self::try_find_etc_passwd(etc_passwd.iter(), |etc_passwd_record| etc_passwd_record.user_identifier == user_identifier)?.ok_or(UserAndGroupChoiceError::UserIdentifierNotPresentInEtcPasswd)?;
		etc_passwd_record_user(etc_passwd_record)
	}

	#[inline(always)]
	fn etc_passwd_record_for_user_name(etc_path: &EtcPath, user_name: &UserName) -> Result<(UserIdentifier, GroupIdentifier), UserAndGroupChoiceError>
	{
		let etc_passwd = EtcPasswd::open(etc_path)?;
		let etc_passwd_record = Self::try_find_etc_passwd(etc_passwd.iter(), |etc_passwd_record| etc_passwd_record.has_user_name(user_name))?.ok_or(UserAndGroupChoiceError::UserNameNotPresentInEtcPasswd)?;
		Ok((etc_passwd_record.user_identifier, etc_passwd_record.group_identifier))
	}

	#[inline(always)]
	fn etc_group_identifier_for_group_name(etc_path: &EtcPath, group_name: &GroupName) -> Result<GroupIdentifier, UserAndGroupChoiceError>
	{
		let etc_group = EtcGroup::open(etc_path)?;
		match Self::try_find_etc_group(etc_group.iter(), |etc_group_record| etc_group_record.has_group_name(group_name))?
		{
			None => Err(UserAndGroupChoiceError::GroupNameNotPresentInEtcGroup),
			Some(etc_group_record) => etc_group_record.group_identifier
		}
	}

	/// `Iterator::try_find()` is unstable, hence the logic here.
	#[inline(always)]
	fn try_find_etc_passwd(iterator: EtcPasswdIterator, predicate: impl FnOnce(&EtcPasswdRecord) -> bool) -> Result<Option<EtcPasswdRecord>, EtcPasswdParseError>
	{
		for etc_passwd_record in iterator
		{
			let etc_passwd_record = etc_passwd_record?;
			if predicate(&etc_passwd_record)
			{
				return Ok(Some(etc_passwd_record))
			}
		}
		Ok(None)
	}

	/// `Iterator::try_find()` is unstable, hence the logic here.
	#[inline(always)]
	fn try_find_etc_group(iterator: EtcPasswdGroup, predicate: impl FnOnce(&EtcGroupRecord) -> bool) -> Result<Option<EtcGroupRecord>, EtcGroupParseError>
	{
		for etc_group_record in iterator
		{
			let etc_group_record = etc_group_record?;
			if predicate(&etc_group_record)
			{
				return Ok(Some(etc_group_record))
			}
		}
		Ok(None)
	}
}
