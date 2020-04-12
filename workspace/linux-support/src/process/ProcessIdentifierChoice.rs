// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process identifier choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessIdentifierChoice
{
	/// The current (Self) process.
	Current,

	/// Another process (potentially, actually, this process).
	Other(ProcessIdentifier),
}

impl Default for ProcessIdentifierChoice
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcessIdentifierChoice::Current
	}
}

impl From<pid_t> for ProcessIdentifierChoice
{
	#[inline(always)]
	fn from(value: pid_t) -> Self
	{
		use self::ProcessIdentifierChoice::*;
		if value == 0
		{
			Current
		}
		else
		{
			Other(ProcessIdentifier::from(unsafe { NonZeroI32::new_unchecked(value) }))
		}
	}
}

impl From<NonZeroI32> for ProcessIdentifierChoice
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		ProcessIdentifierChoice::Other(ProcessIdentifier::from(value))
	}
}

impl Into<pid_t> for ProcessIdentifierChoice
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		use self::ProcessIdentifierChoice::*;
		match self
		{
			Current => 0,
			Other(process_identifier) => process_identifier.into(),
		}
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ProcessIdentifierChoice
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		use self::ProcessIdentifierChoice::*;
		match self
		{
			Current => 0i32.into_line_feed_terminated_byte_string(),
			Other(process_identifier) => process_identifier.into_line_feed_terminated_byte_string(),
		}
	}
}

impl ProcessIdentifierChoice
{
	/// Opens a process identifier file descriptor.
	#[inline(always)]
	pub fn open_file_descriptor(self) -> Result<ProcessIdentifierFileDescriptor, CreationError>
	{
		ProcessIdentifierFileDescriptor::open(self)
	}

	/// Gets the process group identifier (pgid) for this process.
	#[inline(always)]
	pub fn process_group_identifier(self) -> Result<ProcessGroupIdentifier, ()>
	{
		ProcessGroupIdentifier::process_group_identifier(self)
	}

	/// Gets the session identifier (sid) for this process.
	///
	/// The session identifier of a process is the process group identifier of the session leader.
	#[inline(always)]
	pub fn session_identifer(self) -> Result<ProcessGroupIdentifier, ()>
	{
		ProcessGroupIdentifier::session_identifier(self)
	}

	#[inline(always)]
	pub(crate) fn to_file_name(self) -> Cow<'static, str>
	{
		const self_: Cow<'static, str> = Cow::Borrowed("self");

		use self::ProcessIdentifierChoice::*;
		match self
		{
			Current => self_,
			Other(process_identifier) =>
			{
				let value: pid_t = process_identifier.into();
				Cow::from(format!("{}", value))
			},
		}
	}
}
