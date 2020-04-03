// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts the general echo settings.
///
/// Defaults to `Echo::Off`, which is the most sensible choice.
///
/// Echoing is possible in both NonCanonical and Canonical modes.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum Echo
{
	/// No echoing.
	///
	/// This is the default.
	Off = 0,

	/// Echoing is on.
	///
	/// Equivalent to the `ECHO` flag.
	On = ECHO,

	// If set then ASCII control characters (those characters in the range octal 0 through octal 37, inclusive) other than the ASCII `TAB`, the ASCII `N`L, and the `START` and `STOP` characters are echoed as `^X`, where `X` is the character formed by adding octal 100 to the control character.
	///
	/// This means that the ASCII `Control-A` character (octal 1) is echoed as `^A`.
	/// As an exception, the ASCII `DELETE` character (octal 177) is echoed as ^?.
	///
	/// If this flag is not set, the ASCII control characters are echoed as themselves.
	///
	/// Equivalent to the flag combination `ECHO | ECHOCTL`.
	OnWithControlCharacterPrinting = ECHO | ECHOCTL,
}

impl Default for Echo
{
	#[inline(always)]
	fn default() -> Self
	{
		Echo::Off
	}
}

impl Into<tcflag_t> for Echo
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl MultipleBits for Echo
{
	const Bitmask: tcflag_t = ECHO | ECHOCTL | ECHOPRT;

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}
