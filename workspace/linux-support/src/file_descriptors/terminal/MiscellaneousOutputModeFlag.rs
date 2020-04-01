// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Miscellaneous output mode flags.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum MiscellaneousOutputModeFlag
{
	/// Implementation-defined output processing.
	///
	/// (Should normally be disabled).
	///
	/// If this is set, output data is processed in some unspecified way so that it is displayed appropriately on the terminal device.
	/// This typically includes mapping newline characters ('\n') onto carriage return and linefeed pairs.
	///
	/// If this is unset, characters are transmitted as-is.
	ImplementationDefinedOutputProcessing = OPOST,

	/// Map lowercase to uppercase.
	///
	/// This is considered a legacy feature by later POSIX standards.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] MapLowercaseToUppercase = OLCUC,

	/// Map new line, `NL`, (also known as line feed, `\n`) to carriage return, `CR` followed by new line, `NL`, on output.
	MapNewLineToCarriageReturnNewLine = ONLCR,

	/// Map carriage return, `CR` to new line, `NL`, (also known as line feed, `\n`) on output.
	MapCarriageReturnToNewLine = OCRNL,

	/// Discard carriage return, `CR`, at column 0.
	DiscardCarriageReturnAtColumnZero = ONOCR,

	/// Discard carriage return, `CR`.
	DiscardCarriageReturn = ONLRET,

	/// Send fill characters for a delay, rather than using a timed delay.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] SendFillCharactersForADelay = OFILL,

	/// If set, the fill character is ASCII delete, `DEL`, (`0177`).
	///
	/// If unset, the fill character is ASCII `NUL` (`\0') (not implemented on Linux).
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] FillCharacterIsAsciiDeleteRatherThaAsciiNul = OFDEL,

	/// Expands tabs to spaces (with tab stops every eight columns).
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] ExpandTabsToSpaces = XTABS,
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] ExpandTabsToSpaces = OXTABS,

	/// Discard `Ctrl-D` characters (`\x04`) on output.
	///
	/// These characters cause many dial-up terminals to disconnect.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] DiscardControlD = ONOEOT,
}

impl Into<tcflag_t> for MiscellaneousOutputModeFlag
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}
