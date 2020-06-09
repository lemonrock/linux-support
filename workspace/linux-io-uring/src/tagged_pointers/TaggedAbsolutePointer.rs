// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Uses a tagged pointer scheme:-
///
/// * On 64-bit systems, pointers are always 64-bit (8 byte) aligned, so the lowest three bits are always zero.
/// * On x86-64 and AArch64, pointers are 48-bit, so the top 16 bits are available, and, for user space, are always zero.
/// * On x86-64, pointers are technically 47-bit, with the 48th bit (one-based counting) always fixed as zero.
/// * If using pointers to structs which are a power-of-two size, pointers may have a greater alignment so giving more lower bits:-
/// 	* 16-byte aligned data has 4 lowest bits available.
/// 	* 32-byte aligned data has 5 lowest bits available.
/// 	* 64-byte aligned data has 6 lowest bits available.
/// * In the future, Intel will support 52-bit points and Arm 56-bit pointers, but these will be opt-in.
/// * Further bits are available if a pointer is relative to some base.
///
/// However, if we are integrating with `CoroutineInstanceHandle`, the topmost bit can never be set.
#[cfg(target_pointer_width = "64")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct TaggedAbsolutePointer<T>(u64, PhantomData<T>);

impl<T: Sized> TaggedAbsolutePointer<T>
{
	const AbsolutePointerBitMask: u64 = 0b00000000_00000000_01111111_11111111_11111111_11111111_11111111_11111000;
	
	#[inline(always)]
	const fn into_absolute_pointer(self) -> NonNull<T>
	{
		unsafe { NonNull::new_unchecked((self.0 & Self::AbsolutePointerBitMask) as usize as *mut T) }
	}
	
	#[inline(always)]
	pub(crate) const fn low_three_bits(self) -> u8
	{
		(self.0 & b111) as u8
	}
}
