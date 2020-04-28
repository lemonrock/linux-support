// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Etc passwd record.
///
/// Has a lifetime as it shares its data with the underlying file's bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtcGroupRecord<'a>
{
	/// Raw value.
	///
	/// Use `self.group_name()` for a lifetime-independent, but cloned, value.
	pub raw_group_name: &'a [u8],

	#[allow(missing_docs)]
	pub group_identifier: GroupIdentifier,

	raw_user_list: &'a [u8],
}

impl<'a> EtcGroupRecord<'a>
{
	/// Has group name?
	#[inline(always)]
	pub fn has_group_name(&self, group_name: &GroupName) -> bool
	{
		group_name.equals_raw_name(self.raw_user_name)
	}

	/// Clones data.
	#[inline(always)]
	pub fn group_name(&self) -> Result<GroupName, NulError>
	{
		Ok(GroupName(CString::new(self.raw_user_name)?))
	}

	/// User names.
	///
	/// Clones data.
	#[inline(always)]
	pub fn user_names(&self) -> impl Iterator<Item=Result<UserName, NulError>>
	{
		self.raw_user_names().map(|raw_user_name| Ok(UserName(CString::new(raw_user_name)?)))
	}

	/// Raw user names.
	///
	/// Use `self.raw_names()` if cloned data is wanted.
	#[inline(always)]
	pub fn raw_user_names(&self) -> impl Iterator<Item=&'a [u8]>
	{
		self.raw_user_list.split(|byte| *byte == b',')
	}
}
