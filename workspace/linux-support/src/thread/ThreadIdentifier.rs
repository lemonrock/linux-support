// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A thread identifier (`tid`).
///
/// In a single-threaded process, the thread identifier (`tid`) is equal to the process identifier (`pid`) as returned by `ProcessIdentifier::default()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ThreadIdentifier(NonZeroI32);

impl Default for ThreadIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		let tid = gettid.syscall0() as i32;
		debug_assert!(tid > 0);
		Self(unsafe { NonZeroI32::new_unchecked(tid)})
	}
}

impl TryFrom<pid_t> for ThreadIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: pid_t) -> Result<Self, Self::Error>
	{
		if likely!(value > 0)
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(value)}))
		}
		else
		{
			Err(ParseNumberError::TooSmall)
		}
	}
}

impl From<NonZeroI32> for ThreadIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl From<ProcessIdentifier> for ThreadIdentifier
{
	#[inline(always)]
	fn from(value: ProcessIdentifier) -> Self
	{
		Self(value.into())
	}
}

impl Into<NonZeroI32> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl Into<ProcessIdentifier> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> ProcessIdentifier
	{
		ProcessIdentifier::from(self.0)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ThreadIdentifier
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self.0).into_line_feed_terminated_byte_string()
	}
}

impl ThreadIdentifier
{
	#[inline(always)]
	pub(crate) fn to_file_name(self) -> String
	{
		format!("{}", self.0)
	}
}
