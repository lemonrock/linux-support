// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Abstracts the parity.
///
/// See <https://viereck.ch/linux-mark-space-parity/> for workarounds for systems that don't support `Mark` and `Space` parity.
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"), repr(u32))]
#[cfg_attr(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"), repr(u64))]
pub enum Parity
{
	/// Parity disabled.
	///
	/// Also known as `N` or `n` in `8n1` notation.
	Off = 0,

	/// Even parity enabled.
	///
	/// Also known as `E` or `e` in `8e1` notation.
	Even = PARENB,

	/// Odd parity enabled.
	///
	/// Also known as `O` or `o` in `8o1` notation.
	Odd = PARENB | PARODD,

	/// The parity bit is always 1 ('mark').
	///
	/// Also known as `M` or `m` in `8m1` notation.
	///
	/// This seems to also be known as 'stick' parity.
	/// It is not true parity, but an 'abuse' of the parity bit to, say, address nodes on a serial bus.
	///
	/// See answers on [stackoverflow](https://stackoverflow.com/questions/13953095/what-is-the-difference-between-using-mark-space-parity-and-parity-none) for examples.
	///
	/// Actually using it effectively to send packets will require frequently changing the terminal properties and using `TCSADRAIN`.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] Mark = PARENB | PARODD | CMSPAR,

	/// The parity bit is always 0 ('space').
	///
	/// Also known as `S` or `s` in `8s1` notation.
	///
	/// This seems to also be known as 'stick' parity.
	/// It is not true parity, but an 'abuse' of the parity bit to, say, address nodes on a serial bus.
	///
	/// See answers on [stackoverflow](https://stackoverflow.com/questions/13953095/what-is-the-difference-between-using-mark-space-parity-and-parity-none) for examples.
	///
	/// Actually using it effectively to send packets will require frequently changing the terminal properties and using `TCSADRAIN`.
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] Space = PARENB | CMSPAR,
}

impl Into<tcflag_t> for Parity
{
	#[inline(always)]
	fn into(self) -> tcflag_t
	{
		self as tcflag_t
	}
}

impl Default for Parity
{
	#[inline(always)]
	fn default() -> Self
	{
		Parity::Off
	}
}

impl MultipleBits for Parity
{
	#[cfg(any(target_os = "android", target_os = "fuschia", target_os = "linux"))] const Bitmask: tcflag_t = PARENB | PARODD | CMSPAR;
	#[cfg(not(any(target_os = "android", target_os = "fuschia", target_os = "linux")))] const Bitmask: tcflag_t = PARENB | PARODD;

	#[inline(always)]
	fn transmute_from_clean_mode_flags(clean_mode_flags: tcflag_t) -> Self
	{
		unsafe { transmute(clean_mode_flags) }
	}
}
