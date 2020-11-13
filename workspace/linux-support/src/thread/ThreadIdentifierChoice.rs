// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A thread identifier choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ThreadIdentifierChoice
{
	/// The current (Self) thread.
	Current,

	/// Another thread (potentially, actually, this thread).
	Other(ThreadIdentifier),
}

impl Default for ThreadIdentifierChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		ThreadIdentifierChoice::Current
	}
}

impl From<pid_t> for ThreadIdentifierChoice
{
	#[inline(always)]
	fn from(value: pid_t) -> Self
	{
		use self::ThreadIdentifierChoice::*;
		if value == 0
		{
			Current
		}
		else
		{
			Other(ThreadIdentifier::from(new_non_zero_i32(value)))
		}
	}
}

impl From<NonZeroI32> for ThreadIdentifierChoice
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		ThreadIdentifierChoice::Other(ThreadIdentifier::from(value))
	}
}

impl Into<pid_t> for ThreadIdentifierChoice
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		use self::ThreadIdentifierChoice::*;
		match self
		{
			Current => 0,
			Other(thread_identifier) => thread_identifier.into(),
		}
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ThreadIdentifierChoice
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::ThreadIdentifierChoice::*;
		match self
		{
			Current => UnpaddedDecimalInteger(0i32).into_line_feed_terminated_byte_string(),
			Other(thread_identifier) => thread_identifier.into_line_feed_terminated_byte_string(),
		}
	}
}
