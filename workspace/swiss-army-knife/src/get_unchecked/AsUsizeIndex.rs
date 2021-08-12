// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Trivially casts to `usize` without panic.
pub trait AsUsizeIndex
{
	/// Trivially casts to `usize`.
	fn as_usize(self) -> usize;
}

impl AsUsizeIndex for usize
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self
	}
}

impl<'a, AUI: AsUsizeIndex + Copy> AsUsizeIndex for &'a AUI
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		(*self).as_usize()
	}
}

impl AsUsizeIndex for u8
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self as usize
	}
}

impl AsUsizeIndex for u16
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self as usize
	}
}

impl AsUsizeIndex for u32
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self as usize
	}
}

#[cfg(target_pointer_width = "64")]
impl AsUsizeIndex for u64
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self as usize
	}
}

/// Implemented as the default type assigned to an integer by Rust is `i32`, so this makes it possible to use literal numerics for indices.
impl AsUsizeIndex for i32
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		debug_assert!(self >= 0);
		self as usize
	}
}

impl AsUsizeIndex for NonZeroU8
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.get().as_usize()
	}
}

impl AsUsizeIndex for NonZeroU16
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.get().as_usize()
	}
}

impl AsUsizeIndex for NonZeroU32
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.get().as_usize()
	}
}

#[cfg(target_pointer_width = "64")]
impl AsUsizeIndex for NonZeroU64
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.get().as_usize()
	}
}
