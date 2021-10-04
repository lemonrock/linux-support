// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unaligned memory which can be byte swapped.
///
/// Byte swapping works irrespective of whether a value is signed or unsigned.
pub trait ByteSwapUnalignedMemory
{
	/// Converts from little endian to native endian.
	///
	/// Does nothing on a little endian CPU architecture.
	#[inline(always)]
	fn byte_swap_from_little_endian_to_native_endian(&mut self)
	{
		if cfg!(target_endian = "big")
		{
			self.byte_swap()
		}
	}
	
	/// Converts from big endian to native endian.
	///
	/// Does nothing on a big endian CPU architecture.
	#[inline(always)]
	fn byte_swap_from_big_endian_to_native_endian(&mut self)
	{
		if cfg!(target_endian = "little")
		{
			self.byte_swap()
		}
	}
	
	/// Swaps endian irrespective of CPU architecture.
	fn byte_swap(&mut self);
}
