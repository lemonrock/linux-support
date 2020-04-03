// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents settings for local mode flags.
///
/// Note that is is not possible to set the `FLUSHO` flag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalModeFlagSettings
{
	/// Terminal mode.
	pub terminal_mode: Option<TerminalMode>,

	/// Echo settings.
	pub echo: Option<Echo>,

	/// Signal raising behaviour.
	pub signal_raising: Option<SignalRaising>,

	/// Miscellaneous.
	pub miscellaneous: MiscellaneousLocalModeFlagSettings,
}

impl Default for LocalModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			terminal_mode: Some(TerminalMode::default()),
			echo: Some(Echo::default()),
			signal_raising: Some(SignalRaising::default()),
			miscellaneous: MiscellaneousLocalModeFlagSettings::default(),
		}
	}
}

impl LocalModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, terminal_options: &mut termios)
	{
		let existing_flags = terminal_options.c_lflag;

		let mut new_flags = existing_flags;

		if let Some(ref terminal_mode) = self.terminal_mode
		{
			new_flags = terminal_mode.change_mode_flags(new_flags);
		}
		new_flags = MultipleBits::change_mode_flags(self.echo, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.signal_raising, new_flags);
		new_flags = self.miscellaneous.change_mode_flags(new_flags);

		terminal_options.c_lflag = new_flags;
	}
}
