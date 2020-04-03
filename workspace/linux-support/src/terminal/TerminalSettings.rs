// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Terminal settings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerminalSettings
{
	/// Input mode settings.
	pub input_mode_settings: Option<InputModeFlagSettings>,

	/// Output mode settings.
	pub output_mode_settings: Option<OutputModeFlagSettings>,

	/// Control mode settings.
	pub control_mode_settings: Option<ControlModeFlagSettings>,

	/// Local mode settings.
	pub local_mode_settings: Option<LocalModeFlagSettings>,

	/// Character settings.
	pub character_settings: Option<CharacterSettings>,

	/// Baud rate.
	pub baud_rate: Option<BaudRate>,
}

impl TerminalSettings
{
	#[inline(always)]
	pub(crate) fn change_settings(&self, mut terminal_options: &mut termios)
	{
		if let Some(ref input_mode_settings) = self.input_mode_settings
		{
			input_mode_settings.change_mode_flags(&mut terminal_options);
		}
		if let Some(ref output_mode_settings) = self.output_mode_settings
		{
			output_mode_settings.change_mode_flags(&mut terminal_options);
		}
		if let Some(ref control_mode_settings) = self.control_mode_settings
		{
			control_mode_settings.change_mode_flags(&mut terminal_options);
		}
		if let Some(ref local_mode_settings) = self.local_mode_settings
		{
			local_mode_settings.change_mode_flags(&mut terminal_options);
		}
		if let Some(ref character_settings) = self.character_settings
		{
			character_settings.change_character_settings(&mut terminal_options);
		}
		if let Some(ref baud_rate) = self.baud_rate
		{
			baud_rate.set_terminal_input_and_output_speed(&mut terminal_options);
		}
	}
}
