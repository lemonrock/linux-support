// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a system call successful result.
///
/// Has a value between `0x0000_0000_0000_0000` inclusive and `0xFFFF_FFFF_FFFF_F000` inclusive (`0x0000_0000_0000_0000 ..= 0xFFFF_FFFF_FFFF_F000`).
/// In practice, Linux and other operating systems restrict the maximum value to `0x0000_0000_FFFF_F0000` for 32-bit compatibility.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SystemCallResultOkValue(usize);

impl const From<SystemCallResultOkValue> for u64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0 as u64
	}
}

impl const From<SystemCallResultOkValue> for u128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0 as u128
	}
}

impl const From<SystemCallResultOkValue> for usize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0
	}
}

impl const From<SystemCallResultOkValue> for i64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0 as i64
	}
}

impl const From<SystemCallResultOkValue> for i128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0 as i128
	}
}

impl const From<SystemCallResultOkValue> for isize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResultOkValue) -> Self
	{
		system_call_result.0 as isize
	}
}

impl const AsUsizeIndex for SystemCallResultOkValue
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.0
	}
}

impl Step for SystemCallResultOkValue
{
	#[inline(always)]
	fn steps_between(start: &Self, end: &Self) -> Option<usize>
	{
		let start = usize::from(*start);
		let end = usize::from(*end);
		if start <= end
		{
			Some(end - start)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn forward_checked(start: Self, count: usize) -> Option<Self>
	{
		Self::forward_checked_inner(start, count, None, Some)
	}
	
	/// Panics on overflow in debug mode.
	/// Saturates on overflow in release mode.
	#[inline(always)]
	fn forward(start: Self, count: usize) -> Self
	{
		if cfg!(debug_assertions)
		{
			Self::forward_checked(start, count).expect("overflow in `Step::forward`")
		}
		else
		{
			Self::forward_checked_inner(start, count, Self::InclusiveMaximum, |this| this)
		}
	}
	
	#[inline(always)]
	unsafe fn forward_unchecked(start: Self, count: usize) -> Self
	{
		let start = usize::from(start);
		let end = start + count;
		Self::from_valid_usize(end)
	}
	
	#[inline(always)]
	fn backward_checked(start: Self, count: usize) -> Option<Self>
	{
		Self::backward_checked_inner(start, count, None, Some)
	}
	
	/// Panics on underflow in debug mode.
	/// Saturates on underflow in release mode.
	#[inline(always)]
	fn backward(start: Self, count: usize) -> Self
	{
		if cfg!(debug_assertions)
		{
			Self::backward_checked(start, count).expect("overflow in `Step::backward`")
		}
		else
		{
			Self::backward_checked_inner(start, count, Self::InclusiveMinimum, |this| this)
		}
	}
	
	#[inline(always)]
	unsafe fn backward_unchecked(start: Self, count: usize) -> Self
	{
		let end = usize::from(start);
		let start = end - count;
		Self::from_valid_usize(start)
	}
}

impl SystemCallResultOkValue
{
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveMinimum ..= SystemCallResult::InclusiveMaximum`.
	///
	/// `0x0000_0000_0000_0000`.
	/// Maps to ok result value `0`.
	pub const InclusiveMinimum: Self = Self(0);
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveMinimum ..= SystemCallResult::InclusiveMaximum`.
	///
	/// `0xFFFF_FFFF_FFFF_F000`.
	/// Maps to ok result value `1,152,921,504,606,846,720`.
	///
	/// In practice, the maximum used to maintain 32-bit compatibility is `0x0000_0000_FFFF_F000`, or `4,294,963,200`.
	pub const InclusiveMaximum: Self = Self(SystemCallResult::InclusiveErrorRangeStartsFrom.0 - 1);
	
	/// Equivalent to `RangeInclusive` and `RangeToInclusive`.
	///
	/// Value is `Self::InclusiveMinimum .. `.
	pub const RangeFrom: RangeFrom<Self> = RangeFrom
	{
		start: Self::InclusiveMinimum
	};
	
	/// Equivalent to `RangeFrom` and `RangeToInclusive`.
	///
	/// Value is `Self::InclusiveMinimum ..= Self::InclusiveMinimum`.
	pub const RangeInclusive: RangeInclusive<Self> = RangeInclusive::new(Self::InclusiveMinimum, Self::InclusiveMaximum);
	
	/// Equivalent to `RangeFrom` and `RangeInclusive`.
	///
	/// Value is ` ..= Self::InclusiveMinimum`.
	pub const RangeToInclusive: RangeToInclusive<Self> = RangeToInclusive
	{
		end: Self::InclusiveMaximum
	};
	
	#[inline(always)]
	fn forward_checked_inner<R>(start: Self, count: usize, invalid: R, valid: impl FnOnce(Self) -> R) -> R
	{
		let start = usize::from(start);
		let end = start.checked_add(count);
		match end
		{
			None => invalid,
			
			Some(value) if value <= Self::InclusiveMaximum.as_usize() => valid(Self::from_valid_usize(value)),
			
			Some(_) => invalid,
		}
	}
	
	#[inline(always)]
	fn backward_checked_inner<R>(start: Self, count: usize, invalid: R, valid: impl FnOnce(Self) -> R) -> R
	{
		let end = usize::from(start);
		let start = end.checked_sub(count);
		match start
		{
			None => invalid,
			
			Some(0) => invalid,
			
			Some(value) => valid(Self::from_valid_usize(value))
		}
	}
	
	#[inline(always)]
	const fn from_valid_usize(value: usize) -> Self
	{
		if cfg!(debug_assertions)
		{
			if value > Self::InclusiveMaximum.0
			{
				panic!("value is greater than InclusiveMaximum")
			}
		}
		Self(value)
	}
}
