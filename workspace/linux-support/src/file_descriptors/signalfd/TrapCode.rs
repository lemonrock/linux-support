// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGTRAP` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum TrapCode
{
	/// Process breakpoint.
	///
	/// Known as `TRAP_BRKPT` in Linux sources.
	ProcessBreakpoint = 1,

	/// Process trace trap.
	///
	/// Known as `TRAP_TRACE` in Linux sources.
	ProcessTraceTrap = 2,

	/// Process has taken branch trap.
	///
	/// Known as `TRAP_BRANCH` in Linux sources.
	ProcessBranchTrap = 3,

	/// Hardware breakpoint or watchpoint.
	///
	/// Known as `TRAP_HWBKPT` in Linux sources.
	HardwareBreakpointOrWatchpoint = 4,

	/// Undiagnosed trap.
	///
	/// Known as `TRAP_UNK` in Linux sources.
	Undiagnosed = 5,
}

impl Into<i32> for TrapCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for TrapCode
{
	type Data = FaultData;

	const InclusiveMaximum: Self = TrapCode::Undiagnosed;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
