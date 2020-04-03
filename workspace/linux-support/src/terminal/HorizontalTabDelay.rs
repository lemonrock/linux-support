// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts horizonal tab (`HT` or `TAB`, `\t`) delay.
///
/// Defaults to zero.
///
/// Values one and two are only supported on Android, ?Fuschia and Linux.
///
/// Value three is supported on all Android, FreeBSD, ?Fuschia, iOS, Linux, macos and OpenBSD (it is not supported on DragonflyBSD).
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum HorizontalTabDelay
{
	/// Zero.
	#[cfg(not(target_os = "dragonfly"))] Zero = TAB0,
	#[cfg(target_os = "dragonfly")] Zero = 0,

	/// One.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] One = TAB1,

	/// Two.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] Two = TAB2,

	/// Three.
	#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "openbsd"))] Three = TAB3,
}

impl Into<tcflag_t> for HorizontalTabDelay
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for HorizontalTabDelay
{
	#[inline(always)]
	fn default() -> Self
	{
		HorizontalTabDelay::Zero
	}
}

impl MultipleBits for HorizontalTabDelay
{
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "ios", target_os = "linux", target_os = "macos"))] const Bitmask: tcflag_t = TABDLY;

	#[cfg(target_os = "openbsd")]
	#[inline(always)]
	fn from_mode_flags(mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(mode_flags & TAB3) }
	}

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}
