// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts the general signal raising settings.
///
/// Defaults to `SignalRaising::Off`, which is the most sensible choice.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum SignalRaising
{
	/// No echoing.
	///
	/// This is the default.
	Off = 0,

	/// If set, the input characters are compared against the special characters (`INTR`, `QUIT`, `SUSP`, and `DSUSP`) that cause the terminal-generated signals (`SIGINT`, `SIGQUIT`, `SIGSUSP` and `SIGTSTP`) to be generated; if equal, the corresponding signal is generated.
	///
	/// When the terminal driver generates the `SIGINT` and `SIGQUIT` signals (because `RaiseSignals` is set), both the input and output queues are flushed.
	/// When it generates the `SIGSUSP` signal, the input queue is flushed.
	///
	/// On Linux, the signal `SIGTSTP` is *not* raised when the `DSUSP` character is encountered.
	///
	/// On most systems, the following control codes output the associated special characters:-
	///
	/// * `INTR`: `Control-C`.
	/// * `INTR`: `Control-\`.
	/// * `SUSP`: `Control-Z`.
	/// * `DUSP`: `Control-X`.
	///
	/// Equivalent to the `ISIG` flag.
	Raise = ISIG,

	/// Like `SignalRaising::Raise`, but the flusing on input and output queues does not occur.
	///
	/// Equivalent to the flag combination `ISIG | NOFLSH`.
	RaiseWithoutFlush = ISIG | NOFLSH,
}

impl Into<tcflag_t> for SignalRaising
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for SignalRaising
{
	#[inline(always)]
	fn default() -> Self
	{
		SignalRaising::Off
	}
}

impl MultipleBits for SignalRaising
{
	const Bitmask: tcflag_t = ISIG | NOFLSH;

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}
