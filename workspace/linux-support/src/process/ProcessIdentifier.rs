// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ProcessIdentifier(NonZeroI32);

impl Default for ProcessIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		let pid = unsafe { getpid() };
		debug_assert!(pid > 0);
		Self(unsafe { NonZeroI32::new_unchecked(pid)})
	}
}

impl TryFrom<pid_t> for ProcessIdentifier
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

impl From<NonZeroI32> for ProcessIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl Into<NonZeroI32> for ProcessIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ProcessIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ProcessIdentifier
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self.0.into_line_feed_terminated_byte_string()
	}
}

impl ParseNumber for ProcessIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(pid) }))
		}
	}
}

impl ParseNumber for Option<ProcessIdentifier>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if pid == 0
		{
			Ok(None)
		}
		else
		{
			Ok(Some(ProcessIdentifier(unsafe { NonZeroI32::new_unchecked(pid) })))
		}
	}
}

impl ProcessIdentifier
{
	/// Init process.
	pub const Init: Self = Self(unsafe { NonZeroI32::new_unchecked(1) });

	/// Should have a parent process?
	#[inline(always)]
	pub fn should_have_parent(self) -> bool
	{
		self != Self::Init
	}

	/// Opens a process identifier file descriptor.
	#[inline(always)]
	pub fn open_file_descriptor(self) -> Result<ProcessIdentifierFileDescriptor, CreationError>
	{
		ProcessIdentifierChoice::Other(self).open_file_descriptor()
	}
}
