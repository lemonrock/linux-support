// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Choices for output mode flag control.
///
/// Default does nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MiscellaneousLocalModeFlagSettings(BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>);

impl Default for MiscellaneousLocalModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(BTreeMap::new())
	}
}

impl From<BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>> for MiscellaneousLocalModeFlagSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>> for MiscellaneousLocalModeFlagSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>
	{
		self.0
	}
}

impl Deref for MiscellaneousLocalModeFlagSettings
{
	type Target = BTreeMap<MiscellaneousLocalModeFlag, FlagSetting>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for MiscellaneousLocalModeFlagSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl MiscellaneousLocalModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, existing_flags: tcflag_t) -> tcflag_t
	{
		use self::FlagSetting::*;

		let mut flags_on = 0;
		let mut flags_off = 0;

		for (flag, setting) in self.0.iter()
		{
			let flag_value = (*flag) as tcflag_t;

			match setting
			{
				On => flags_on |= flag_value,
				Off => flags_off |= flag_value,
			}
		}

		(existing_flags | flags_on) & !flags_off
	}

	#[inline(always)]
	pub(crate) fn from_mode_flags(mode_flags: tcflag_t) -> Self
	{
		let mut this = Self(BTreeMap::new());

		for flag in MiscellaneousLocalModeFlag::iter()
		{
			this.insert_flag_setting(flag, mode_flags);
		}

		this
	}

	#[inline(always)]
	fn insert_flag_setting(&mut self, miscellaneous_control_mode_flag: MiscellaneousLocalModeFlag, control_mode_flags: tcflag_t)
	{
		let flag_setting = FlagSetting::from((miscellaneous_control_mode_flag as tcflag_t) & control_mode_flags != 0);
		self.insert(miscellaneous_control_mode_flag, flag_setting);
	}
}
