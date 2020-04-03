// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Current terminal settings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrentTerminalSettings(pub(crate) termios);

impl CurrentTerminalSettings
{
	/// Input mode flag settings.
	#[inline(always)]
	pub fn input_mode_flag_settings(&self) -> InputModeFlagSettings
	{
		InputModeFlagSettings::from_mode_flags(self.0.c_iflag)
	}

	/// Backspace delay.
	#[inline(always)]
	pub fn backspace_delay(&self) -> BackspaceDelay
	{
		BackspaceDelay::from_mode_flags(self.0.c_oflag)
	}

	/// Carriage Return delay.
	#[inline(always)]
	pub fn carriage_return_delay(&self) -> CarriageReturnDelay
	{
		CarriageReturnDelay::from_mode_flags(self.0.c_oflag)
	}

	/// Form Feed delay.
	#[inline(always)]
	pub fn form_feed_delay(&self) -> FormFeedDelay
	{
		FormFeedDelay::from_mode_flags(self.0.c_oflag)
	}

	/// Horizontal Tab delay.
	#[inline(always)]
	pub fn horizontal_tab_delay(&self) -> HorizontalTabDelay
	{
		HorizontalTabDelay::from_mode_flags(self.0.c_oflag)
	}

	/// New Line delay.
	#[inline(always)]
	pub fn new_line_delay(&self) -> NewLineDelay
	{
		NewLineDelay::from_mode_flags(self.0.c_oflag)
	}

	/// Vertical Tab delay.
	#[inline(always)]
	pub fn vertical_tab_delay(&self) -> VerticalTabDelay
	{
		VerticalTabDelay::from_mode_flags(self.0.c_oflag)
	}

	/// Miscellaneous output mode flag settings.
	#[inline(always)]
	pub fn miscellaneous_output_mode_flag_settings(&self) -> MiscellaneousOutputModeFlagSettings
	{
		MiscellaneousOutputModeFlagSettings::from_mode_flags(self.0.c_oflag)
	}

	/// Bits per byte.
	#[inline(always)]
	pub fn bits_per_byte(&self) -> BitsPerByte
	{
		BitsPerByte::from_mode_flags(self.0.c_cflag)
	}

	/// How many stop bits?
	#[inline(always)]
	pub fn stop_bits(&self) -> StopBits
	{
		StopBits::from_mode_flags(self.0.c_cflag)
	}

	/// Parity.
	#[inline(always)]
	pub fn parity(&self) -> Parity
	{
		Parity::from_mode_flags(self.0.c_cflag)
	}

	/// Miscellaneous control mode flag settings.
	#[inline(always)]
	pub fn miscellaneous_control_mode_flag_settings(&self) -> MiscellaneousControlModeFlagSettings
	{
		MiscellaneousControlModeFlagSettings::from_mode_flags(self.0.c_cflag)
	}

	/// Miscellaneous local mode flag settings.
	#[inline(always)]
	pub fn miscellaneous_local_mode_flag_settings(&self) -> MiscellaneousLocalModeFlagSettings
	{
		MiscellaneousLocalModeFlagSettings::from_mode_flags(self.0.c_lflag)
	}

	/// Is flushing occurring?
	#[inline(always)]
	pub fn is_flushing_occurring(&self) -> bool
	{
		self.0.c_lflag & FLUSHO != 0
	}

	/// Terminal Mode.
	#[inline(always)]
	pub fn terminal_mode(&self) -> TerminalMode
	{
		TerminalMode::interpret_terminal_mode(self.0.c_lflag)
	}

	/// Echo.
	#[inline(always)]
	pub fn echo(&self) -> Echo
	{
		Echo::from_mode_flags(self.0.c_lflag)
	}

	/// Signal raising behaviour.
	#[inline(always)]
	pub fn signal_raising(&self) -> SignalRaising
	{
		SignalRaising::from_mode_flags(self.0.c_lflag)
	}

	/// Character settings.
	#[inline(always)]
	pub fn character_settings(&self) -> CharacterSettings
	{
		CharacterSettings::from_terminal_options(&self.0)
	}

	/// Baud rate.
	#[inline(always)]
	pub fn baud_rate(&self) -> BaudRate
	{
		unsafe { transmute(cfgetospeed(&self.0)) }
	}

}
