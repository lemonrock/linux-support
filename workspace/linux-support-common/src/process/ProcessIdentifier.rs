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
		Self(unsafe { NonZeroI32::new_unchecked(1) })
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
		Ok(Self(NonZeroI32::parse_number(bytes, radix, parse_byte)?))
	}
}

impl ParseNumber for Option<ProcessIdentifier>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if pid == 0
		{
			Ok(None)
		}
		else
		{
			Ok(Some(ProcessIdentifier(unsafe { NonZeroI32::new_unchecked(pid) })))
		}
	}
}
