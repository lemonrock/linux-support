// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) trait NonNullAtomicU32Ext
{
	fn store_release(self, new_value: u32);

	fn load_acquire(self) -> u32;
}

impl NonNullAtomicU32Ext for NonNull<AtomicU32>
{
	#[inline(always)]
	fn store_release(self, new_value: u32)
	{
		(unsafe { self.as_ref() }).store(new_value, Release)
	}

	#[inline(always)]
	fn load_acquire(self) -> u32
	{
		(unsafe { self.as_ref() }).load(Acquire)
	}
}
