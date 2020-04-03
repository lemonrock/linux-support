// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Choices for output mode flag control.
///
/// Default is sane for a raw terminal; no mapping, implementation defined output processing is disabled, no tab expansion, no discard of characters and fill character settings are inherited.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MiscellaneousOutputModeFlagSettings(BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>);

impl Default for MiscellaneousOutputModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::MiscellaneousOutputModeFlag::*;
		use self::FlagSetting::*;

		let mut this = Self(BTreeMap::new());
		this.insert(ImplementationDefinedOutputProcessing, Off);
		#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] this.insert(MapLowercaseToUppercase, Off);
		this.insert(MapNewLineToCarriageReturnNewLine, Off);
		this.insert(MapCarriageReturnToNewLine, Off);
		this.insert(DiscardCarriageReturnAtColumnZero, Off);
		this.insert(DiscardCarriageReturn, Off);
		// SendFillCharactersForADelay is inherited.
		// FillCharacterIsAsciiDeleteRatherThaAsciiNul is inherited.
		this.insert(ExpandTabsToSpaces, Off);
		#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] this.insert(DiscardControlD, Off);

		this
	}
}

impl From<BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>> for MiscellaneousOutputModeFlagSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>> for MiscellaneousOutputModeFlagSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>
	{
		self.0
	}
}

impl Deref for MiscellaneousOutputModeFlagSettings
{
	type Target = BTreeMap<MiscellaneousOutputModeFlag, FlagSetting>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for MiscellaneousOutputModeFlagSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl MiscellaneousOutputModeFlagSettings
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

		for flag in MiscellaneousOutputModeFlag::iter()
		{
			this.insert_flag_setting(flag, mode_flags);
		}

		this
	}

	#[inline(always)]
	fn insert_flag_setting(&mut self, miscellaneous_output_mode_flag: MiscellaneousOutputModeFlag, output_mode_flags: tcflag_t)
	{
		let flag_setting = FlagSetting::from((miscellaneous_output_mode_flag as tcflag_t) & output_mode_flags != 0);
		self.insert(miscellaneous_output_mode_flag, flag_setting);
	}
}
