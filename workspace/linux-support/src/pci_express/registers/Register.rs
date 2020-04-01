// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Register.
pub trait Register<RS: RegisterSize>: Sized
{
	#[doc(hidden)]
	#[inline(always)]
	fn new(base_pointer: NonNull<u8>, offset: usize) -> Self
	{
		let pointer = (base_pointer.as_ptr() as usize) + offset;
		Self::new_internal(RS::as_non_null(pointer))
	}

	#[doc(hidden)]
	fn new_internal(pointer: NonNull<RS>) -> Self;

	/// Read.
	fn read(&self) -> RS;

	/// Spin-wait until applying `bitmask` to the register value gives `value`.
	///
	/// If `value` is not given then until all bits in the mask are set.
	#[inline(always)]
	fn wait(&self, bitmask: RS, value: Option<RS>)
	{
		let compare_to = value.unwrap_or(bitmask);

		while (self.read() & bitmask) != compare_to
		{
			const OneHundredMicroseconds: Duration = Duration::from_micros(100);
			sleep(OneHundredMicroseconds);
		}
	}

	/// Reset (no-op for registers that are not counters).
	fn reset(&self);

	/// Gets bits.
	#[inline(always)]
	fn bits(&self, start: RS, length: RS) -> RS
	{
		self.read().bits(start, length)
	}

	/// Gets a byte.
	#[inline(always)]
	fn byte(&self, start: RS) -> RS
	{
		self.bits(start * RS::Eight, RS::Eight)
	}
}
