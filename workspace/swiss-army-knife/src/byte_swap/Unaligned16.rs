// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An unaligned 16-bit value, usually an integer (signed or unsigned).
pub type Unaligned16 = [u8; 2];

impl ByteSwapUnalignedMemory for [Unaligned16]
{
	#[inline(always)]
	fn byte_swap(&mut self)
	{
		Unaligned16::byte_swap_unaligned_memory(self)
	}
}

impl Unaligned for Unaligned16
{
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
	const ShuffleControlMask128: [i8; BytesVector128Size] =
	[
		14, 15,
		12, 13,
		10, 11,
		8, 9,
		6, 7,
		4, 5,
		2, 3,
		0, 1
	];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
	const ShuffleControlMask256: [i8; BytesVector256Size] =
	[
		30, 31,
		28, 29,
		26, 27,
		24, 25,
		22, 23,
		20, 21,
		18, 19,
		16, 17,
		
		14, 15,
		12, 13,
		10, 11,
		8, 9,
		6, 7,
		4, 5,
		2, 3,
		0, 1,
	];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
	const ShuffleControlMask512: [i8; BytesVector512Size] =
	[
		62, 63,
		60, 61,
		58, 59,
		56, 57,
		54, 55,
		52, 53,
		50, 51,
		48, 49,
		
		46, 47,
		44, 45,
		42, 43,
		40, 41,
		38, 39,
		36, 37,
		34, 35,
		32, 33,
		
		30, 31,
		28, 29,
		26, 27,
		24, 25,
		22, 23,
		20, 21,
		18, 19,
		16, 17,
		
		14, 15,
		12, 13,
		10, 11,
		8, 9,
		6, 7,
		4, 5,
		2, 3,
		0, 1,
	];
	
	type Aligned = u16;
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "movbe"))]
	#[inline(always)]
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned
	{
		load_movbe_16(self)
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
