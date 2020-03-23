// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read-only and return the sum of all values read.
///
/// This is for counter registers that clear back to zero when read.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReadOnlyCounterResetOnReadRegister<RS: RegisterSize>
{
	pointer: NonNull<RS>,
	accumulator: Cell<RS>,
}

impl<RS: RegisterSize> Register<RS> for ReadOnlyCounterResetOnReadRegister<RS>
{
	#[inline(always)]
	fn new_internal(pointer: NonNull<RS>) -> Self
	{
		Self
		{
			pointer,
			accumulator: Cell::new(RS::Zero),
		}
	}

	#[inline(always)]
	fn read(&self) -> RS
	{
		let read_value = unsafe { read(self.pointer.as_ptr()) };
		let new_total = self.accumulator.get() + read_value;
		self.accumulator.set(new_total);
		return new_total
	}

	#[inline(always)]
	fn reset(&self)
	{
		self.accumulator.set(RS::Zero);
	}
}
