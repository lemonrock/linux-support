// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File access permissions.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AccessPermissions(mode_t);

impl From<u16> for AccessPermissions
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from(value as u32)
	}
}

impl From<mode_t> for AccessPermissions
{
	#[inline(always)]
	fn from(value: mode_t) -> Self
	{
		Self(value)
	}
}

impl AccessPermissions
{
	/// Special permissions.
	#[inline(always)]
	pub fn special_permissions(self) -> SpecialPermissions
	{
		unsafe { transmute(((self.0 >> 12) & S_IRWXO) as u8) }
	}

	/// User accessibility.
	#[inline(always)]
	pub fn user(self) -> Accessibility
	{
		unsafe { transmute(((self.0 & S_IRWXU) >> 8) as u8) }
	}

	/// Group accessibility.
	#[inline(always)]
	pub fn group(self) -> Accessibility
	{
		unsafe { transmute(((self.0 & S_IRWXG) >> 4) as u8) }
	}

	/// All (other) accessibility.
	#[inline(always)]
	pub fn other(self) -> Accessibility
	{
		unsafe { transmute((self.0 & S_IRWXO) as u8) }
	}
}
