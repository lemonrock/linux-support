// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// User data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CoroutineUserData(u64);

impl UserData for CoroutineUserData
{
	#[inline(always)]
	fn into_u64(self) -> u64
	{
		self.0
	}
	
	#[inline(always)]
	fn from_u64(value: u64) -> Self
	{
		Self(value)
	}
}

impl CoroutineUserData
{
	const IsCoroutineBit: u64 = 0x8000_0000_0000_0000;
	
	const OriginalRequestCancelationKindMask: u64 = 0x6000_0000_0000_0000;
	
	const OriginalRequestCancelationKindMaskShift: u64 = 61;
	
	const CoroutineIndexMask: u64 = 0x0000_0000_FFFF_FFF0;
	
	const CoroutineIndexShift: u64 = 4;
	
	const ExclusiveMaximumCoroutineIndex: usize = ((Self::CoroutineIndexMask >> Self::CoroutineIndexShift) + 1) as usize;
	
	#[inline(always)]
	pub fn from_coroutine_operation_slot_index(original_request_cancelation_kind: OriginalRequestCancelationKind, coroutine_index: usize, operation_slot_index: usize) -> Self
	{
		debug_assert!(coroutine_index < Self::ExclusiveMaximumCoroutineIndex);
		debug_assert!(operation_slot_index < CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots);
		
		Self(Self::IsCoroutineBit | ((original_request_cancelation_kind as u8 as u64) << Self::OriginalRequestCancelationKindMaskShift) | ((operation_slot_index as u64) << Self::CoroutineIndexShift) | (operation_slot_index as u64))
	}
	
	#[inline(always)]
	pub const fn is_for_coroutine(self) -> bool
	{
		(self.0 & Self::IsCoroutineBit) != 0
	}
	
	/// `0 .. 3`.
	#[inline(always)]
	pub const fn original_request_cancelation_kind(self) -> OriginalRequestCancelationKind
	{
		unsafe { transmute(((self.0 & Self::OriginalRequestCancelationKindMask) >> Self::OriginalRequestCancelationKindMaskShift) as u8) }
	}
	
	/// Check `is_for_coroutine()` first.
	///
	/// `0 .. (2 ^ 28)`.
	#[inline(always)]
	pub const fn coroutine_index(self) -> usize
	{
		((self.0 & Self::CoroutineIndexMask) >> Self::CoroutineIndexShift) as usize
	}
	
	/// Check `is_for_coroutine()` first.
	///
	/// `0 .. CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots` (where `CoroutineParallelOperationSlots::ExclusiveMaximumOperationSlots` is currently 16).
	#[inline(always)]
	pub const fn coroutine_operation_slot_index(self) -> usize
	{
		(self.0 & 0x0000_0000_0000_000F) as usize
	}
}
