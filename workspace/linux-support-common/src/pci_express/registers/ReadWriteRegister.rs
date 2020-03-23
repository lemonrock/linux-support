// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read-write.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadWriteRegister<RS: RegisterSize>
{
	pointer: NonNull<RS>,
}

impl<RS: RegisterSize> Register<RS> for ReadWriteRegister<RS>
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

impl<RS: RegisterSize> ReadWriteRegister<RS>
{
	/// Write.
	#[inline(always)]
	pub fn write(&self, value: RS) -> RS
	{
		unsafe { write(self.pointer.as_ptr(), value) };
		value
	}

	/// Write bits.
	#[inline(always)]
	pub fn write_bits(&self, start: RS, length: RS, bits: RS) -> RS
	{
		let tmp = self.read();
		let offmask = !(RS::bitmask(length) << start);
		let tmp = tmp & offmask;
		let tmp = tmp | (bits << start);
		self.write(tmp)
	}

	/// Write byte.
	///
	/// Set a byte length bytes from an offset of start bytes.
	#[inline(always)]
	pub fn write_byte(&self, start: RS, byte: RS) -> RS
	{
		self.write_bits(start * RS::Eight, RS::Eight, byte)
	}

	/// Set bits.
	#[inline(always)]
	pub fn set(&self, bitmask: RS) -> RS
	{
		self.write(self.read() | bitmask)
	}

	/// Clear bits.
	#[inline(always)]
	pub fn clr(&self, bitmask: RS) -> RS
	{
		self.write(self.read() & !bitmask)
	}
}
