// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts the canonical settings of a terminal.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalSettings
{
	/// If set, the terminal is assumed to be uppercase only, and all input is converted to lowercase.
	///
	/// To input an uppercase character, precede it with a backslash.
	/// Similarly, an uppercase character is output by the system by being preceded by a backslash.
	///
	/// This option flag is obsolete today, since most, if not all, uppercase-only terminals have disappeared.
	///
	/// Equivalent to the `XCASE` flag.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] pub convert_uppercase_to_lowercase: bool,

	/// If set then the `ERASE` character erases the last character in the current line from the display.
	///
	/// This is usually done in the terminal driver by writing the three-character sequence 'backspace, space, backspace'.
	///
	/// If the word erase, `WERASE`, character is supported then this setting causes the previous word to be erased using one or more of the same three-character sequence when `WERASE` is received.
	///
	/// This occurs irrespective of whether the terminal has been set to echo its input.
	///
	/// Equivalent to the `ECHOE` flag.
	pub echo_erase: bool,

	/// If set then the new line, `NL`, character is echoed.
	///
	/// This occurs irrespective of whether the terminal has been set to echo its input.
	///
	/// Equivalent to the `ECHONL` flag.
	pub echo_new_line: bool,

	///  If set, canonical character processing is performed external to the operating system.
	///
	/// This can be the case if the serial communication peripheral card can offload the host processor by doing some of the line discipline processing.
	///
	/// Equivalent to the `EXTPROC` flag.
	pub external_processing: bool,

	/// If set then the `ERASE` character (and `WERASE` character, if supported) will cause all the characters being erased to be printed as they are erased.
	///
	/// This is often useful on a hard-copy terminal to see exactly which characters are being deleted.
	///
	/// Setting this flag ***will force echo to be on***, overriding `Echo::Off`.
	///
	/// Equivalent to the `ECHOPRT` flag.
	pub print_before_erase: bool,

	/// Different choices for echoing the `KILL` character.
	pub echo_kill_character: CanonicalEchoKillCharacter,
}

impl CanonicalSettings
{
	#[inline(always)]
	pub(crate) fn change_mode_flags_assuming_no_bits_set(&self, new_flags: tcflag_t) -> tcflag_t
	{
		let mut bits_to_set = 0;

		#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))]
		{
			if unlikely!(self.convert_uppercase_to_lowercase)
			{
				bits_to_set |= XCASE;
			}
		}

		if self.echo_erase
		{
			bits_to_set |= ECHOE;
		}


		if self.echo_new_line
		{
			bits_to_set |= ECHONL;
		}


		if self.external_processing
		{
			bits_to_set |= EXTPROC;
		}

		if self.print_before_erase
		{
			bits_to_set |= ECHO | ECHOPRT;
		}

		bits_to_set |= self.echo_kill_character as tcflag_t;


		new_flags | bits_to_set
	}


	#[inline(always)]
	pub(crate) fn interpret_mode_flags(current_flags: tcflag_t) -> Self
	{
		CanonicalSettings
		{
			#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] convert_uppercase_to_lowercase: current_flags & XCASE != 0,
			echo_erase: current_flags & ECHOE != 0,
			echo_new_line: current_flags & ECHONL != 0,
			external_processing: current_flags & EXTPROC != 0,
			print_before_erase: current_flags & (ECHO | ECHOPRT) != 0,
			echo_kill_character: CanonicalEchoKillCharacter::interprete_mode_flags(current_flags)
		}
	}
}
