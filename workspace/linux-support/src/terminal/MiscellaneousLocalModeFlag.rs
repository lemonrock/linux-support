// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Miscellaneous local mode flags.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum MiscellaneousLocalModeFlag
{
	/// If set, the extended, implementation-defined special characters (such as `WERASE`) are recognized and processed.
	///
	/// Equivalent to the `IEXTEN` flag.
	ImplementationDefinedOutputProcessing = IEXTEN,

	///  If set and if the implementation supports job control, the `SIGTTOU` signal is sent to the process group of a background process that tries to write to its controlling terminal.
	///
	/// By default, this signal stops all the processes in the process group.
	///
	/// This signal is not generated by the terminal driver if the background process that is writing to the controlling terminal is either ignoring or blocking the signal.
	///
	/// Equivalent to the `TOSTOP` flag.
	RaiseSigTTouSignal = TOSTOP,

	/// If set then any input that has not been read is reprinted by the system when the next character is input.
	///
	/// This action is similar to what happens when the `REPRINT` character is typed.
	///
	/// Equivalent to the `PENDIN` flag.
	ReprintUnreadInput = PENDIN,

	/// If set then this flag prevents the `STATUS` character from printing information on the foreground process group.
	///
	/// Regardless of this flag, however, the `STATUS` character still causes the `SIGINFO` signal to be sent to the foreground process group.
	///
	/// Equivalent to the `NOKERNINFO` flag.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] PreventStatusCharacterFromPrintingInformation = NOKERNINFO,

	/// If set then an alternate word-erase algorithm is used when the `WERASE` character is entered.
	///
	/// Instead of moving backward until the previous white space character, this flag causes the `WERASE` character to move backward until the first non-alphanumeric character is encountered.
	///
	/// Equivalent to the `ALTWERASE` flag.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "openbsd"))] AlternativeWordEraseAlgorithm = ALTWERASE,
}

impl Into<tcflag_t> for MiscellaneousLocalModeFlag
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}
