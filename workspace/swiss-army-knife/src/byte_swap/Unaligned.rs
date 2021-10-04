// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Byte swap (change endian order of) an array of unaligned memory.
///
/// Byte swapping works irrespective of whether a value is signed or unsigned.
pub trait Unaligned: Debug + Copy + Eq + Ord + Hash
{
	#[doc(hidden)]
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
	const ShuffleControlMask128: [i8; BytesVector128Size];
	
	#[doc(hidden)]
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
	const ShuffleControlMask256: [i8; BytesVector256Size];
	
	#[doc(hidden)]
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
	const ShuffleControlMask512: [i8; BytesVector512Size];
	
	/// Converts from little endian to native endian.
	///
	/// Does nothing on a little endian CPU architecture.
	#[inline(always)]
	fn byte_swap_unaligned_memory_from_little_endian_to_native_endian(unaligned_memory: &mut [Self])
	{
		if cfg!(target_endian = "big")
		{
			Self::byte_swap_unaligned_memory(unaligned_memory)
		}
	}
	
	/// Converts from big endian to native endian.
	///
	/// Does nothing on a big endian CPU architecture.
	#[inline(always)]
	fn byte_swap_unaligned_memory_from_big_endian_to_native_endian(unaligned_memory: &mut [Self])
	{
		if cfg!(target_endian = "little")
		{
			Self::byte_swap_unaligned_memory(unaligned_memory)
		}
	}
	
	/// Byte swap unaligned memory.
	#[allow(unused_mut)]
	#[inline(always)]
	fn byte_swap_unaligned_memory(mut unaligned_memory: &mut [Self])
	{
		debug_assert_eq!(size_of::<Self>(), size_of::<Self::Aligned>());
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
		{
			unaligned_memory = BytesVector512::byte_swap_unaligned_memory::<Self>(unaligned_memory, Self::ShuffleControlMask512);
		}
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
		{
			unaligned_memory = BytesVector256::byte_swap_unaligned_memory::<Self>(unaligned_memory, Self::ShuffleControlMask256);
		}
		
		#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
		{
			unaligned_memory = BytesVector128::byte_swap_unaligned_memory::<Self>(unaligned_memory, Self::ShuffleControlMask128);
		}
		
		Self::swap_remaining_using_architecture_byte_swap_instruction(unaligned_memory)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn swap_remaining_using_architecture_byte_swap_instruction(unaligned_memory: &mut [Self])
	{
		for index in 0 .. unaligned_memory.len()
		{
			let unaligned = unaligned_memory.get_unchecked_mut_safe(index);
			unaligned.swap_using_architecture_byte_swap_instruction()
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn swap_using_architecture_byte_swap_instruction(&mut self)
	{
		let aligned = self.load_and_swap_from_any_endian_bytes();
		*self = Self::into_any_endian_bytes(aligned)
	}
	
	#[doc(hidden)]
	type Aligned: Copy;
	
	#[doc(hidden)]
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned;
	
	#[doc(hidden)]
	fn into_any_endian_bytes(aligned: Self::Aligned) -> Self;
	
	#[doc(hidden)]
	fn pointer(&self) -> *const Self::Aligned;
}
