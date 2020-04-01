// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read-only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadOnlyRegister<RS: RegisterSize>
{
	pointer: NonNull<RS>,
}

impl<RS: RegisterSize> Register<RS> for ReadOnlyRegister<RS>
{
	#[inline(always)]
	fn new_internal(pointer: NonNull<RS>) -> Self
	{
		Self
		{
			pointer
		}
	}

	#[inline(always)]
	fn read(&self) -> RS
	{
		unsafe { read(self.pointer.as_ptr()) }
	}

	#[inline(always)]
	fn reset(&self)
	{
	}
}
