// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// What mode the terminal is in.
///
/// Defaults to `TerminalMode::NonCanonical`, which is the most sensible choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TerminalMode
{
	/// Also known as raw.
	///
	/// This is the default.
	NonCanonical,

	/// Canonical mode terminals process input and output, and are typically used when the terminal is visually interactive.
	Canonical(CanonicalSettings),
}

impl Default for TerminalMode
{
	#[inline(always)]
	fn default() -> Self
	{
		TerminalMode::NonCanonical
	}
}

impl TerminalMode
{
	const CommonCanonicalBitmask: tcflag_t = ICANON | ECHOE | ECHONL | EXTPROC | ECHOK | ECHOKE | ECHOPRT;
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] const CanonicalBitmask: tcflag_t = Self::CommonCanonicalBitmask | XCASE;
	#[cfg(not(any(target_os = "android", target_os = "fuschia", target_os = "linux")))] const CanonicalBitmask: tcflag_t = Self::CommonCanonicalBitmask;

	#[inline(always)]
	pub(crate) fn change_mode_flags(&self, current_flags: tcflag_t) -> tcflag_t
	{
		use self::TerminalMode::*;

		let new_flags = current_flags & !Self::CanonicalBitmask;

		match self
		{
			&NonCanonical => new_flags,

			&Canonical(ref canonical_settings) => canonical_settings.change_mode_flags_assuming_no_bits_set(new_flags),
		}
	}

	#[inline(always)]
	pub(crate) fn interpret_terminal_mode(current_flags: tcflag_t) -> Self
	{
		use self::TerminalMode::*;

		if current_flags & ICANON == 0
		{
			NonCanonical
		}
		else
		{
			Canonical(CanonicalSettings::interpret_mode_flags(current_flags))
		}
	}
}
