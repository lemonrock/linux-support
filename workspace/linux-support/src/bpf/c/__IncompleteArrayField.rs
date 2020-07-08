// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default)]
pub(crate) struct __IncompleteArrayField<T>(PhantomData<T>, [T; 0]);

impl<T> Debug for __IncompleteArrayField<T>
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result
	{
		fmt.write_str("__IncompleteArrayField")
	}
}

impl<T> __IncompleteArrayField<T>
{
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		__IncompleteArrayField(PhantomData, [])
	}
}
