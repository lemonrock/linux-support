// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

/// A marker trait for the various process identifiers.
pub trait ProcessIdentifier
{
	#[doc(hidden)]
	const Current: Cow<'static, str> = Cow::Borrowed("self");

	#[doc(hidden)]
	fn Other(value: i32) -> Cow<'static, str>
	{
		Cow::from(format!("{}", value))
	}

	/// To a file name.
	fn to_file_name(self) -> Cow<'static, str>;
}

impl ProcessIdentifier for pid_t
{
	#[inline(always)]
	fn to_file_name(self) -> Cow<'static, str>
	{
		if self == 0
		{
			Self::Current
		}
		else
		{
			Self::Other(self)
		}
	}
}

impl ProcessIdentifier for NonZeroI32
{
	#[inline(always)]
	fn to_file_name(self) -> Cow<'static, str>
	{
		Self::Other(self.get())
	}
}

impl ProcessIdentifier for Option<NonZeroI32>
{
	#[inline(always)]
	fn to_file_name(self) -> Cow<'static, str>
	{
		match self
		{
			None => Self::Current,
			Some(value) => Self::Other(value.get()),
		}
	}
}

