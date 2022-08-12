// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Etc passwd record.
///
/// Has a lifetime as it shares its data with the underlying file's bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtcPasswdRecord<'a>
{
	/// Raw value.
	///
	/// Use `self.user_name()` for a lifetime-independent, but cloned, value.
	pub raw_user_name: &'a [u8],

	#[allow(missing_docs)]
	pub user_identifier: UserIdentifier,

	#[allow(missing_docs)]
	pub group_identifier: GroupIdentifier,

	/// GECOS ("General Electric Comprehensive Operating System", also known as GECOS when owned by Honeywell) or comment field.
	///
	/// One of the original designers of Unix, Denis Ritchie, has, paraphrased, called this field 'not elegant' - it was originally a hack.
	///
	/// Often empty, might be the same as `UserName` or comma-separated, eg `Linux User,,,`.
	pub gecos: &'a [u8],

	/// Raw value.
	///
	/// Use `self.user_name()` for a lifetime-independent, but cloned, value.
	pub raw_home_directory: &'a [u8],

	/// Raw value.
	///
	/// Use `self.user_name()` for a lifetime-independent, but cloned, value.
	pub raw_shell: &'a [u8],
}

impl<'a> EtcPasswdRecord<'a>
{
	/// Has user name?
	#[inline(always)]
	pub fn has_user_name(&self, user_name: &UserName) -> bool
	{
		user_name.equals_raw_name(self.raw_user_name)
	}

	/// Used as the value of the `USER` environment variable.
	///
	/// Clones data.
	#[inline(always)]
	pub fn user_name(&self) -> Result<UserName, NulError>
	{
		Ok(UserName(CString::new(self.raw_user_name)?))
	}

	/// Used as the value of the `HOME` environment variable.
	///
	/// Clones data.
	#[inline(always)]
	pub fn home_directory(&self) -> PathBuf
	{
		Self::to_path_buf(self.raw_home_directory)
	}

	/// Used as the value of the `SHELL` environment variable.
	///
	/// Clones data.
	#[inline(always)]
	pub fn shell(&self) ->  PathBuf
	{
		Self::to_path_buf(self.raw_shell)
	}

	#[inline(always)]
	fn to_path_buf(value: &'a [u8]) -> PathBuf
	{
		PathBuf::from(OsString::from_vec(value.to_vec()))
	}
}
