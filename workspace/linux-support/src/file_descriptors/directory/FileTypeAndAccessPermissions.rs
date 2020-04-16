// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File type and access permissions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTypeAndAccessPermissions(mode_t);

impl From<u16> for FileTypeAndAccessPermissions
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from(value as u32)
	}
}

impl From<mode_t> for FileTypeAndAccessPermissions
{
	#[inline(always)]
	fn from(value: mode_t) -> Self
	{
		Self(value)
	}
}

impl Into<mode_t> for FileTypeAndAccessPermissions
{
	#[inline(always)]
	fn into(self) -> mode_t
	{
		self.0
	}
}

impl FileTypeAndAccessPermissions
{
	/// File type.
	#[inline(always)]
	pub const fn file_type(self) -> FileType
	{
		FileType::const_from(self.0)
	}

	/// Access permissions.
	#[inline(always)]
	pub const fn access_permissions(self) -> AccessPermissions
	{
		AccessPermissions(self.0)
	}
}
