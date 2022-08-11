// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a system call result.
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SystemCallResult(usize);

impl const From<SystemCallResult> for u64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as u64
	}
}

impl const From<SystemCallResult> for u128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as u128
	}
}

impl const From<SystemCallResult> for usize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0
	}
}

impl const From<SystemCallResult> for i64
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as i64
	}
}

impl const From<SystemCallResult> for i128
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as i128
	}
}

impl const From<SystemCallResult> for isize
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.0 as isize
	}
}

impl const AsUsizeIndex for SystemCallResult
{
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.0
	}
}

impl const From<SystemCallResult> for Result<SystemCallResultOkValue, SystemCallErrorNumber>
{
	#[inline(always)]
	fn from(system_call_result: SystemCallResult) -> Self
	{
		system_call_result.into_result()
	}
}

impl SystemCallResult
{
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveOkRangeStartsFrom ..= SystemCallResult::InclusiveOkRangeEndsAt`.
	///
	/// `0x0000_0000_0000_0000`.
	/// Maps to ok result value `0`.
	pub const InclusiveOkRangeStartsFrom: Self = Self(SystemCallResultOkValue::InclusiveMinimum.0);
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveOkRangeStartsFrom ..= SystemCallResult::InclusiveOkRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F000`.
	/// Maps to ok result value `1,152,921,504,606,846,720`.
	/// In musl libc, this is defined as `-4096UL`; the Rust equivalent would be `4096usize.wrapping_neg()`.
	///
	/// In practice, the maximum used to maintain 32-bit compatibility is `0x0000_0000_FFFF_F000`, or `4,294,963,200`.
	pub const InclusiveOkRangeEndsAt: Self = Self(SystemCallResultOkValue::InclusiveMaximum.0);
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_F001`.
	/// Maps to error number `4095`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMaximum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeStartsFrom: Self = Self(-isize::from(SystemCallErrorNumber::InclusiveMaximum) as usize);
	
	/// Use this value in `match` inclusive range clauses as `SystemCallResult::InclusiveErrorRangeStartsFrom ..= SystemCallResult::InclusiveErrorRangeEndsAt`.
	///
	/// `0xFFFF_FFFF_FFFF_FFFF`.
	/// Maps to error number `1`.
	//
	// NOTE: This really is the value `SystemCallErrorNumber::InclusiveMinimum`, as error numbers are returned using two's complement arithmetic.
	pub const InclusiveErrorRangeEndsAt: Self = Self(-isize::from(SystemCallErrorNumber::InclusiveMinimum) as usize);
	
	/// Range of `OK` values (inclusive start, exclusive end).
	/// Equivalent to `OkRangeInclusive`, `OkRangeTo` and `OkRangeToInclusive`.
	///
	/// Value is `Self::InclusiveOkRangeStartsFrom .. Self::InclusiveErrorRangeStartsFrom`.
	pub const OkRange: Range<Self> = Range
	{
		start: Self::InclusiveOkRangeStartsFrom,
		
		end: Self::InclusiveErrorRangeStartsFrom,
	};
	
	/// Range of `OK` values (inclusive start, inclusive end).
	/// Equivalent to `OkRange` and `OkRangeTo`.
	///
	/// Value is `Self::InclusiveOkRangeStartsFrom ..= Self::InclusiveOkRangeEndsAt.
	pub const OkRangeInclusive: RangeInclusive<Self> = RangeInclusive::new(Self::InclusiveOkRangeStartsFrom, Self::InclusiveOkRangeEndsAt);
	
	/// Range of `OK` values (exclusive end).
	/// Equivalent to `OkRange`, `OkRangeInclusive` and `OkRangeToInclusive`.
	///
	/// Value is ` .. Self::InclusiveErrorRangeStartsFrom`.
	pub const OkRangeTo: RangeTo<Self> = RangeTo
	{
		end: Self::InclusiveErrorRangeStartsFrom,
	};
	
	/// Range of `OK` values (exclusive end).
	/// Equivalent to `OkRange`, `OkRangeInclusive` and `OkRangeTo`.
	///
	/// Value is ` ..= Self::InclusiveOkRangeEndsAt`.
	pub const OkRangeToInclusive: RangeToInclusive<Self> = RangeToInclusive
	{
		end: Self::InclusiveOkRangeEndsAt,
	};
	
	/// Range of `Error` values (inclusive start, inclusive end).
	/// Equivalent to `ErrorRangeFrom`.
	///
	/// Value is `Self::InclusiveErrorRangeStartsFrom ..= Self::InclusiveErrorRangeEndsAt`.
	pub const ErrorRangeInclusive: RangeInclusive<Self> = RangeInclusive::new(Self::InclusiveErrorRangeStartsFrom, Self::InclusiveErrorRangeEndsAt);
	
	/// Range of `Error` values (inclusive start).
	/// Equivalent to `ErrorRangeInclusive`.
	///
	/// Value is `Self::InclusiveErrorRangeStartsFrom ..`.
	pub const ErrorRangeFrom: RangeFrom<Self> = RangeFrom
	{
		start: Self::InclusiveErrorRangeStartsFrom
	};
	
	/// Is this OK?
	#[inline(always)]
	pub const fn is_ok(self) -> bool
	{
		self.0 < Self::InclusiveErrorRangeStartsFrom.0
	}
	
	/// Is this an error?
	#[inline(always)]
	pub const fn is_error(self) -> bool
	{
		// There is no need to check for the minimum (`-1`) as it is `0xFFFF_FFFF_FFFF_FFFF`.
		self.0 >= Self::InclusiveErrorRangeStartsFrom.0
	}
	
	/// Sets the `errno` "global static" (usually a value in the `pthread` struct pointed to be `pthread_self()`).
	///
	/// Result of the function is one of:-
	///
	/// * `-1` error number is set.
	/// * `0 ..= isize::MAX`: Valid result.
	/// * `-4096 ..= isize::MIN`: Valid result.
	///
	/// The result of the function can never be `-2 to -4095`.
	///
	/// This horrible approach is because Linux stuffs an error code into an usize register, unlike, say, the BSDs, which return a value in one register and a flag in another indicating if the value is an error or not.
	#[inline(always)]
	pub fn set_errno_to_be_compatible_with_libc(self) -> isize
	{
		if unlikely!(self.is_error())
		{
			self.system_call_error_number().set_errno();
			-1
		}
		else
		{
			self.0 as isize
		}
	}
	
	// TODO: Problem with this design is fast-return, as it is not macro based.
	#[inline(always)]
	pub fn success_is_zero<R>(self, success: impl FnOnce() -> R, error: impl FnOnce(SystemCallErrorNumber) -> R) -> R
	{
		if likely!(self.is_ok())
		{
			if likely!(self.0 == 0)
			{
			}
		}
		else
		{
			Err(self.system_call_error_number())
		}
	}
	
	#[inline(always)]
	const fn into_result(self) -> Result<SystemCallResultOkValue, SystemCallErrorNumber>
	{
		if unlikely!(self.is_error())
		{
			Err(self.system_call_error_number())
		}
		else
		{
			Ok(SystemCallResultOkValue(self.0))
		}
	}
	
	#[inline(always)]
	const fn system_call_error_number(self) -> SystemCallErrorNumber
	{
		SystemCallErrorNumber::from_valid_u16((-(self.0 as isize)) as u16)
	}
}
