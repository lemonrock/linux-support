// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGCHLD` signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ChildCode
{
	/// Child has exited.
	///
	/// Known as `CLD_EXITED` in Linux sources.
	Exited = 1,

	/// Child was killed.
	///
	/// Known as `CLD_KILLED` in Linux sources.
	Killed = 2,

	/// Child terminated abnormally.
	///
	/// Known as `CLD_DUMPED` in Linux sources.
	TerminatedAbnormally = 3,

	/// Traced child has trapped.
	///
	/// Known as `CLD_TRAPPED` in Linux sources.
	Trapped = 4,

	/// Child has stopped.
	///
	/// 'Stopped' in this case refers to process control.
	///
	/// Known as `CLD_STOPPED` in Linux sources.
	Stopped = 5,

	/// Child has continued.
	///
	/// 'Continued' in this case refers to process control.
	///
	/// Known as `CLD_CONTINUED` in Linux sources.
	Continued = 6,
}

impl Into<i32> for ChildCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for ChildCode
{
	type Data = ChildData;

	const InclusiveMaximum: Self = ChildCode::Continued;

	#[inline(always)]
	fn convert(code: i32) -> Self
	{
		unsafe { transmute(code) }
	}
}
