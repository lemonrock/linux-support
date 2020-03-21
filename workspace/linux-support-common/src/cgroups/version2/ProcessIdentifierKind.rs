// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process identifier kind.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProcessIdentifierKind
{
	/// The process identifier for the current process.
	Current,

	/// A known process identifier,
	Known(NonZeroU32),
}

impl Into<u32> for ProcessIdentifierKind
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::ProcessIdentifierKind::*;

		match self
		{
			Current => 0,
			Known(value) => value.get(),
		}
	}
}

impl From<u32> for ProcessIdentifierKind
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		use self::ProcessIdentifierKind::*;

		if unlikely!(value == 0)
		{
			Current
		}
		else
		{
			Known(unsafe { NonZeroU32::new_unchecked(value) })
		}
	}
}
