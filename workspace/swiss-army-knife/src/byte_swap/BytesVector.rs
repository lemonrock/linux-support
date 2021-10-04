// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


trait BytesVector: Debug + Copy
{
	/// Actually, this is `[i8; size_of::<Self>()]` but Rust does not permit this.
	type ShuffleControlMaskBytes: Sized;
	
	#[inline(always)]
	fn byte_swap_unaligned_memory<U: Unaligned>(unaligned_memory: &mut [U], shuffle_control_mask_bytes: Self::ShuffleControlMaskBytes) -> &mut [U]
	{
		// This is a pseudo-constant; Rust does not permit constants to be defined using a generic parameter.
		let BytesVectorSize = size_of::<Self>();
		debug_assert_eq!(BytesVectorSize, size_of::<Self::ShuffleControlMaskBytes>());
		
		// This is a pseudo-constant; Rust does not permit constants to be defined using a generic parameter.
		let NumberOfLanes = BytesVectorSize / size_of::<U>();
		
		let length = unaligned_memory.len();
		if unlikely!(length < NumberOfLanes)
		{
			return unaligned_memory
		}
		
		// This is a pseudo-constant; Rust does not have a constant implementation x86_64 intrinsics that set the bytes in a SIMD register.
		let ShuffleControlMask = Self::create_shuffle_control_mask(shuffle_control_mask_bytes);
		
		let mut bytes_vector_pointer = new_non_null(unaligned_memory.as_mut_ptr() as *mut Self);
		let number_of_bytes_vector = length % NumberOfLanes;
		let bytes_vector_end_pointer = Self::add(bytes_vector_pointer, number_of_bytes_vector);
		while likely!(bytes_vector_pointer != bytes_vector_end_pointer)
		{
			let bytes_vector = Self::load_unaligned_bytes_vector(bytes_vector_pointer);
			let byte_swapped_bytes_vector = bytes_vector.shuffle_bytes_vector(ShuffleControlMask);
			byte_swapped_bytes_vector.store_unaligned_bytes_vector(bytes_vector_pointer);
			
			bytes_vector_pointer = Self::add(bytes_vector_pointer, 1);
		}
		
		unaligned_memory.get_unchecked_range_mut_safe(NumberOfLanes .. )
	}
	
	#[inline(always)]
	fn add(bytes_vector_pointer: NonNull<Self>, count: usize) -> NonNull<Self>
	{
		new_non_null(unsafe { bytes_vector_pointer.as_ptr().add(count) })
	}
	
	fn create_shuffle_control_mask(shuffle_control_mask_bytes: Self::ShuffleControlMaskBytes) -> Self;
	
	fn load_unaligned_bytes_vector(from_memory_address: NonNull<Self>) -> Self;
	
	// Rust permits `ShuffleControlMask` to be const, but it can not be created as a const.
	fn shuffle_bytes_vector(self, ShuffleControlMask: Self) -> Self;
	
	fn store_unaligned_bytes_vector(self, to_memory_address: NonNull<Self>);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "ssse3"))]
impl BytesVector for BytesVector128
{
	type ShuffleControlMaskBytes = [i8; 16];
	
	#[inline(always)]
	fn create_shuffle_control_mask(shuffle_control_mask_bytes: Self::ShuffleControlMaskBytes) -> Self
	{
		let [e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0] = shuffle_control_mask_bytes;
		
		unsafe { set_bytes_vector_128(e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0) }
	}
	
	#[inline(always)]
	fn load_unaligned_bytes_vector(from_memory_address: NonNull<Self>) -> Self
	{
		unsafe { load_unaligned_bytes_vector_128(from_memory_address.as_ptr() as _) }
	}
	
	#[inline(always)]
	fn shuffle_bytes_vector(self, ShuffleControlMask: Self) -> Self
	{
		unsafe { shuffle_bytes_vector_128(self, ShuffleControlMask) }
	}
	
	#[inline(always)]
	fn store_unaligned_bytes_vector(self, to_memory_address: NonNull<Self>)
	{
		unsafe { store_unaligned_bytes_vector_128(to_memory_address.as_ptr() as _, self) }
	}
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
impl BytesVector for BytesVector256
{
	type ShuffleControlMaskBytes = [i8; 32];
	
	#[inline(always)]
	fn create_shuffle_control_mask(shuffle_control_mask_bytes: Self::ShuffleControlMaskBytes) -> Self
	{
		let [e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0] = shuffle_control_mask_bytes;
		
		unsafe { set_bytes_vector_256(e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0) }
	}
	
	#[inline(always)]
	fn load_unaligned_bytes_vector(from_memory_address: NonNull<Self>) -> Self
	{
		unsafe { load_unaligned_bytes_vector_256(from_memory_address.as_ptr() as _) }
	}
	
	#[inline(always)]
	fn shuffle_bytes_vector(self, ShuffleControlMask: Self) -> Self
	{
		unsafe { shuffle_bytes_vector_256(self, ShuffleControlMask) }
	}
	
	#[inline(always)]
	fn store_unaligned_bytes_vector(self, to_memory_address: NonNull<Self>)
	{
		unsafe { store_unaligned_bytes_vector_256(to_memory_address.as_ptr() as _, self) }
	}
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
impl BytesVector for BytesVector512
{
	type ShuffleControlMaskBytes = [i8; 64];
	
	#[inline(always)]
	fn create_shuffle_control_mask(shuffle_control_mask_bytes: Self::ShuffleControlMaskBytes) -> Self
	{
		let [e63, e62, e61, e60, e59, e58, e57, e56, e55, e54, e53, e52, e51, e50, e49, e48, e47, e46, e45, e44, e43, e42, e41, e40, e39, e38, e37, e36, e35, e34, e33, e32, e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0] = shuffle_control_mask_bytes;
		
		unsafe { set_bytes_vector_512(e63, e62, e61, e60, e59, e58, e57, e56, e55, e54, e53, e52, e51, e50, e49, e48, e47, e46, e45, e44, e43, e42, e41, e40, e39, e38, e37, e36, e35, e34, e33, e32, e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0) }
	}
	
	#[inline(always)]
	fn load_unaligned_bytes_vector(from_memory_address: NonNull<Self>) -> Self
	{
		unsafe { load_unaligned_bytes_vector_512(from_memory_address.as_ptr() as *const i8) }
	}
	
	#[inline(always)]
	fn shuffle_bytes_vector(self, ShuffleControlMask: Self) -> Self
	{
		unsafe { shuffle_bytes_vector_512(self, ShuffleControlMask) }
	}
	
	#[inline(always)]
	fn store_unaligned_bytes_vector(self, to_memory_address: NonNull<Self>)
	{
		unsafe { store_unaligned_bytes_vector_512(to_memory_address.as_ptr() as *mut i8, self) }
	}
}
