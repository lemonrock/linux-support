// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Constructs a new instance without checking the provided `value` is correct.
///
/// Used for constructing constants.
pub trait FromUnchecked<T>: Sized
{
	/// Used for constructing constants.
	unsafe fn from_unchecked(value: T) -> Self;
}

impl<T> const FromUnchecked<T> for T
where T: ~const From<T>
{
	#[inline(always)]
	unsafe fn from_unchecked(value: T) -> Self
	{
		T::from(value)
	}
}
