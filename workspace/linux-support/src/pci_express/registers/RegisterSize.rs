// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Register size: 32-bit or 64-bit.
pub trait RegisterSize: Sized + Copy + Clone + BitOr<Output=Self> + BitAnd<Output=Self> + Not<Output=Self> + Shr<Output=Self> + Shl<Output=Self> + Sub<Output=Self> + Add<Output=Self> + Mul<Output=Self>
+ PartialEq + Eq
{
	#[doc(hidden)]
	const Zero: Self;

	#[doc(hidden)]
	const One: Self;

	#[doc(hidden)]
	const Eight: Self;

	#[doc(hidden)]
	const Maximum: Self;

	#[doc(hidden)]
	#[inline(always)]
	fn bitmask(length: Self) -> Self
	{
		(Self::One << length) - Self::One
	}

	#[doc(hidden)]
	#[inline(always)]
	fn bits(self, start: Self, length: Self) -> Self
	{
		(self >> start) & Self::bitmask(length)
	}

	#[doc(hidden)]
	fn as_non_null(pointer: usize) -> NonNull<Self>;
}

impl RegisterSize for u32
{
	const Zero: Self = 0;

	const One: Self = 1;

	const Eight: Self = 8;

	const Maximum: Self = 0xFFFF_FFFF;

	#[inline(always)]
	fn as_non_null(pointer: usize) -> NonNull<Self>
	{
		new_non_null(pointer as *mut Self)
	}
}

impl RegisterSize for u64
{
	const Zero: Self = 0;

	const One: Self = 1;

	const Eight: Self = 8;

	const Maximum: Self = 0xFFFF_FFFF_FFFF_FFFF;

	#[inline(always)]
	fn as_non_null(pointer: usize) -> NonNull<Self>
	{
		new_non_null(pointer as *mut Self)
	}
}
