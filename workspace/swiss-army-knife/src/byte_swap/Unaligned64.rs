// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An unaligned 64-bit value, either an integer (signed or unsigned) or a 64-bit IEEE-754 value.
pub type Unaligned64 = [u8; 8];

impl ByteSwapUnalignedMemory for [Unaligned64]
{
	#[inline(always)]
	fn byte_swap(&mut self)
	{
		Unaligned64::byte_swap_unaligned_memory(self)
	}
}

impl Unaligned for Unaligned64
{
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
	const ShuffleControlMask128: [i8; BytesVector128Size] =
	[
		 8,  9, 10, 11, 12, 13, 14, 15,
		 0,  1,  2,  3,  4,  5,  6,  7,
	];
	
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
	const ShuffleControlMask256: [i8; BytesVector256Size] =
	[
		24, 25, 26, 27, 28, 29, 30, 31,
		16, 17, 18, 19, 20, 21, 22, 23,
		
		8,  9, 10, 11, 12, 13, 14, 15,
		0,  1,  2,  3,  4,  5,  6,  7,
	];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
	const ShuffleControlMask512: [i8; BytesVector512Size] =
	[
		56, 57, 58, 59, 60, 61, 62, 63,
		48, 49, 50, 51, 52, 53, 54, 55,
		
		40, 41, 42, 43, 44, 45, 46, 47,
		32, 33, 34, 35, 36, 37, 38, 39,
		
		24, 25, 26, 27, 28, 29, 30, 31,
		16, 17, 18, 19, 20, 21, 22, 23,
		
		8,  9, 10, 11, 12, 13, 14, 15,
		0,  1,  2,  3,  4,  5,  6,  7,
	];
	
	type Aligned = u64;
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "movbe"))]
	#[inline(always)]
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned
	{
		load_movbe_64(self)
	}
	
	#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "movbe")))]
	#[inline(always)]
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned
	{
		let aligned = unsafe { self.pointer().read_unaligned() };
		aligned.swap_bytes()
	}
	
	
	#[inline(always)]
	fn into_any_endian_bytes(aligned: Self::Aligned) -> Self
	{
		unsafe { transmute(aligned) }
	}
	
	#[inline(always)]
	fn pointer(&self) -> *const Self::Aligned
	{
		self.as_ptr() as *const Self::Aligned
	}
}
