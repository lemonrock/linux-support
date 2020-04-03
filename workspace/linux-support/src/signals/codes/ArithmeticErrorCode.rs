// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGFPE` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ArithmeticErrorCode
{
	/// Integer division by zero.
	///
	/// Known as `FPE_INTDIV` in Linux sources.
	IntegerDivisionByZero = 1,

	/// Integer overflow.
	///
	/// Known as `FPE_INTOVF` in Linux sources.
	IntegerOverflow = 2,

	/// Floating Point division by zero.
	///
	/// Known as `FPE_FLTDIV` in Linux sources.
	FloatingPointDivisionByZero = 3,

	/// Floating Point overflow.
	///
	/// Known as `FPE_FLTOVF` in Linux sources.
	FloatingPointOverflow = 4,

	/// Floating Point underflow.
	///
	/// Known as `FPE_FLTUND` in Linux sources.
	FloatingPointUnderflow = 5,

	/// Floating Point inexact result.
	///
	/// Known as `FPE_FLTRES` in Linux sources.
	FloatingPointInexactResult = 6,

	/// Floating Point invalid operation.
	///
	/// Known as `FPE_FLTINV` in Linux sources.
	FloatingPointInvalidOperation = 7,

	/// Floating Point subscript out of range.
	///
	/// Known as `FPE_FLTSUB` in Linux sources.
	FloatingPointSubscriptOutOfRange = 8,

	/// Packed Decimal overflow.
	///
	/// Known as `__FPE_DECOVF` in Linux sources.
	PackedDecimalOverflow = 9,

	/// Packed Decimal division by zero.
	///
	/// Known as `__FPE_DECOVF` in Linux sources.
	PackedDecimalDivisionBZero = 10,

	/// Packed Decimal error.
	///
	/// Known as `__FPE_DECERR` in Linux sources.
	PackedDecimalError = 11,

	/// Invalid ASCII digit.
	///
	/// Known as `__FPE_INVASC` in Linux sources.
	InvalidAsciiDigit = 12,

	/// Invalid Decimal digit.
	///
	/// Known as `__FPE_INVDEC` in Linux sources.
	InvalidDecimalDigit = 13,

	/// Undiagnosed floating point exception.
	///
	/// Known as `FPE_FLTUNK` in Linux sources.
	UndiagnosedFloatingPointException = 14,

	/// Trap on Condition
	///
	/// Known as `FPE_CONDTRAP` in Linux sources.
	TrapOnCondition = 15,
}

impl Into<i32> for ArithmeticErrorCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for ArithmeticErrorCode
{
	/// Known as `NSIGFPE` in Linux sources.
	const InclusiveMaximum: Self = ArithmeticErrorCode::TrapOnCondition;

	#[inline(always)]
	fn rehydrate(validated_si_code: i32) -> Self
	{
		unsafe { transmute(validated_si_code)}
	}
}
