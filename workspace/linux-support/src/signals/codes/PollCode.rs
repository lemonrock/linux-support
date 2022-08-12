// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a code that can be associated with a kernel-raised `SIGIO` (also known as `SIGPOLL`) signal.
///
/// Definitions valid as of Linux v4.20-rc5.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum PollCode
{
	/// Data input available.
	///
	/// Known as `POLL_IN` in Linux sources.
	Input = 1,

	/// Data output available.
	///
	/// Known as `POLL_OUT` in Linux sources.
	Output = 2,

	/// Input message available.
	///
	/// Known as `POLL_MSG` in Linux sources.
	InputPosixMessage = 3,

	/// Input / Output (IO) error.
	///
	/// Known as `POLL_IN` in Linux sources.
	IoError = 4,

	/// Priority input available (eg TCP 's deprecated out-of-band urgent data).
	///
	/// Known as `POLL_PRI` in Linux sources.
	PriorityInput = 5,

	/// Device disconnected (Hung Up).
	///
	/// Known as `POLL_HUP` in Linux sources.
	HungUp = 6,
}

impl Into<i32> for PollCode
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Code for PollCode
{
	/// Known as `NSIGPOLL` in Linux sources.
	const InclusiveMaximum: Self = PollCode::HungUp;

	#[inline(always)]
	fn rehydrate(validated_si_code: i32) -> Self
	{
		unsafe { transmute(validated_si_code)}
	}
}
