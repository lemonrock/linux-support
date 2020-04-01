// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents settings for output mode flags.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ControlModeFlagSettings
{
	/// Bits per byte.
	pub bits_per_byte: Option<BitsPerByte>,

	/// How many stop bits?
	pub stop_bits: Option<StopBits>,

	/// Parity.
	pub parity: Option<Parity>,

	/// Miscellaneous.
	pub miscellaneous: MiscellaneousControlModeFlagSettings,
}

impl Default for ControlModeFlagSettings
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			bits_per_byte: Some(BitsPerByte::default()),
			stop_bits: Some(StopBits::default()),
			parity: Some(Parity::default()),
			miscellaneous: MiscellaneousControlModeFlagSettings::default(),
		}
	}
}

impl ControlModeFlagSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, terminal_options: &mut termios)
	{
		let existing_flags = terminal_options.c_cflag;

		let mut new_flags = existing_flags;
		new_flags = MultipleBits::change_mode_flags(self.bits_per_byte, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.stop_bits, new_flags);
		new_flags = MultipleBits::change_mode_flags(self.parity, new_flags);
		new_flags = self.miscellaneous.change_mode_flags(new_flags);

		terminal_options.c_cflag = new_flags;
	}
}
