// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Choices for input mode flag control.
///
/// Default is sane for a raw terminal (no XON/XOFF flow control on output, no mapping or ignoring of characters, 8-bit clean, no signal interrupts, no ignoring of break conditions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputModeFlagSettings(BTreeMap<InputModeFlag, FlagSetting>);

impl Default for InputModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::InputModeFlag::*;
		use self::FlagSetting::*;

		let mut this = Self(BTreeMap::new());

		this.insert(IgnoreBreakCondition, Off);
		this.insert(SignalInterruptOnBreak, Off);
		this.insert(MarkParityErrors, Off);
		this.insert(StripOffEighthBitOfEveryCharacter, Off);
		this.insert(MapNewLineToCarriageReturn, Off);
		this.insert(IgnoreCarriageReturn, Off);
		this.insert(MapCarriageReturnToNewLine, Off);
		this.insert(EnableXOnXOffFlowControlOnOutput, Off);
		this.insert(AnyCharacterToRestartOutput, Off);
		#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] this.insert(MapUppercaseToLowercase, Off);
		this.insert(RingBellWhenInputQueueIsFull, Off);
		#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] this.insert(Utf8, Off);

		this
	}
}

impl From<BTreeMap<InputModeFlag, FlagSetting>> for InputModeFlagSettings
{
	#[inline(always)]
	fn from(map: BTreeMap<InputModeFlag, FlagSetting>) -> Self
	{
		Self(map)
	}
}

impl Into<BTreeMap<InputModeFlag, FlagSetting>> for InputModeFlagSettings
{
	#[inline(always)]
	fn into(self) -> BTreeMap<InputModeFlag, FlagSetting>
	{
		self.0
	}
}

impl Deref for InputModeFlagSettings
{
	type Target = BTreeMap<InputModeFlag, FlagSetting>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for InputModeFlagSettings
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl InputModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, terminal_options: &mut termios)
	{
		let existing_flags: tcflag_t = terminal_options.c_iflag;

		let new_flags =
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
		};

		terminal_options.c_iflag = new_flags;
	}

	#[inline(always)]
	pub(crate) fn from_mode_flags(mode_flags: tcflag_t) -> Self
	{
		let mut this = Self(BTreeMap::new());

		for flag in InputModeFlag::iter()
		{
			this.insert_flag_setting(flag, mode_flags);
		}

		this
	}

	#[inline(always)]
	fn insert_flag_setting(&mut self, input_mode_flag: InputModeFlag, input_mode_flags: tcflag_t)
	{
		let flag_setting = FlagSetting::from((input_mode_flag as tcflag_t) & input_mode_flags != 0);
		self.insert(input_mode_flag, flag_setting);
	}
}
