// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An unaligned 32-bit value, either an integer (signed or unsigned) or a 32-bit IEEE-754 value.
pub type Unaligned32 = [u8; 4];

impl ByteSwapUnalignedMemory for [Unaligned32]
{
	#[inline(always)]
	fn byte_swap(&mut self)
	{
		Unaligned32::byte_swap_unaligned_memory(self)
	}
}

impl Unaligned for Unaligned32
{
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
	const ShuffleControlMask128: [i8; BytesVector128Size] =
	[
		12, 13, 14, 15,
		 8,  9, 10, 11,
		 4,  5,  6,  7,
		 0,  1,  2,  3,
	];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
	const ShuffleControlMask256: [i8; BytesVector256Size] =
	[
		28, 29, 30, 31,
		24, 25, 26, 27,
		20, 21, 22, 23,
		16, 17, 18, 19,
		
		12, 13, 14, 15,
		8,  9, 10, 11,
		4,  5,  6,  7,
		0,  1,  2,  3,
	];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
	const ShuffleControlMask512: [i8; BytesVector512Size] =
	[
		60, 61, 62, 63,
		56, 57, 58, 59,
		52, 53, 54, 55,
		48, 49, 50, 51,
		
		44, 45, 46, 47,
		40, 41, 42, 43,
		36, 37, 38, 39,
		32, 33, 34, 35,
		
		28, 29, 30, 31,
		24, 25, 26, 27,
		20, 21, 22, 23,
		16, 17, 18, 19,
		
		12, 13, 14, 15,
		8,  9, 10, 11,
		4,  5,  6,  7,
		0,  1,  2,  3,
	];
	
	type Aligned = u32;
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "movbe"))]
	#[inline(always)]
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned
	{
		load_movbe_32(self)
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
