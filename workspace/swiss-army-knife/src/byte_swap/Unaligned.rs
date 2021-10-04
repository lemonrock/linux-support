// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This trait exists because Rust's standard library does not abstract methods common across primitive integers into a trait, eg `fn swap_bytes()`.
trait Unaligned: Debug + Copy + Eq + Ord + Hash
{
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
	const ShuffleControlMask128: [i8; BytesVector128Size];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
	const ShuffleControlMask256: [i8; BytesVector256Size];
	
	#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
	const ShuffleControlMask512: [i8; BytesVector512Size];
	
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
	
	#[inline(always)]
	fn swap_remaining_using_architecture_byte_swap_instruction(unaligned_memory: &mut [Self])
	{
		for index in 0 .. unaligned_memory.len()
		{
			let unaligned = unaligned_memory.get_unchecked_mut_safe(index);
			unaligned.swap_using_architecture_byte_swap_instruction()
		}
	}
	
	#[inline(always)]
	fn swap_using_architecture_byte_swap_instruction(&mut self)
	{
		let aligned = self.load_and_swap_from_any_endian_bytes();
		*self = Self::into_any_endian_bytes(aligned)
	}
	
	type Aligned: Copy;
	
	/// Should be optimized to use the `MOVBE` instruction when available on x86 and x86_64.
	fn load_and_swap_from_any_endian_bytes(&self) -> Self::Aligned;
	
	fn into_any_endian_bytes(aligned: Self::Aligned) -> Self;
	
	fn pointer(&self) -> *const Self::Aligned;
}
