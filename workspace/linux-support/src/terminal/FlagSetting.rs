// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// On or off.
///
/// Default is `FlagSetting::Off`.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlagSetting
{
	/// Set on.
	On,

	/// Set off.
	Off,
}

impl Into<bool> for FlagSetting
{
	#[inline(always)]
	fn into(self) -> bool
	{
		use self::FlagSetting::*;

		match self
		{
			On => true,
			Off => false,
		}
	}
}

impl From<bool> for FlagSetting
{
	#[inline(always)]
	fn from(value: bool) -> Self
	{
		use self::FlagSetting::*;

		match value
		{
			true => On,
			false => Off,
		}
	}
}

impl Default for FlagSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		FlagSetting::Off
	}
}
