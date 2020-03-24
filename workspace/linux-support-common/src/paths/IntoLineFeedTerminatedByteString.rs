// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Converts data to a byte string terminated with a new line (`\n`).
pub trait IntoLineFeedTerminatedByteString<'a>
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>;
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a [u8]
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(self)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for Vec<u8>
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(self)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a Vec<u8>
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		Cow::from(&self[..])
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for bool
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let value = if self
		{
			b"1\n"
		}
		else
		{
			b"0\n"
		};
		Cow::from(value as &[u8])
	}
}
