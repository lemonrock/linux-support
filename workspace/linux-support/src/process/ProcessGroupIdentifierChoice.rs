// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process group identifier choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessGroupIdentifierChoice
{
	/// The current (Self) process.
	Current,

	/// Another process (potentially, actually, this process).
	Other(ProcessGroupIdentifier),
}

impl Default for ProcessGroupIdentifierChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcessGroupIdentifierChoice::Current
	}
}

impl From<pid_t> for ProcessGroupIdentifierChoice
{
	#[inline(always)]
	fn from(value: pid_t) -> Self
	{
		use self::ProcessGroupIdentifierChoice::*;
		if value == 0
		{
			Current
		}
		else
		{
			Other(ProcessGroupIdentifier::from(new_non_zero_i32(value)))
		}
	}
}

impl From<NonZeroI32> for ProcessGroupIdentifierChoice
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		ProcessGroupIdentifierChoice::Other(ProcessGroupIdentifier::from(value))
	}
}

impl Into<pid_t> for ProcessGroupIdentifierChoice
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		use self::ProcessGroupIdentifierChoice::*;
		match self
		{
			Current => 0,
			Other(process_identifier) => process_identifier.into(),
		}
	}
}
