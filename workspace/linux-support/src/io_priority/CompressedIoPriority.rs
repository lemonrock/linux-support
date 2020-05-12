// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Compressed into a `u16`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CompressedIoPriority(u16);

impl From<IoPriority> for CompressedIoPriority
{
	#[inline(always)]
	fn from(value: IoPriority) -> Self
	{
		CompressedIoPriority(value.into())
	}
}

impl From<Option<IoPriority>> for CompressedIoPriority
{
	#[inline(always)]
	fn from(value: Option<IoPriority>) -> Self
	{
		match value
		{
			None => Self::Irrelevant,
			Some(value) => Self::from(value)
		}
	}
}

impl Into<Option<IoPriority>> for CompressedIoPriority
{
	#[inline(always)]
	fn into(self) -> Option<IoPriority>
	{
		IoPriority::parse_ioprio(self.0).unwrap()
	}
}

impl CompressedIoPriority
{
	/// Irrelevant.
	pub const Irrelevant: Self = Self(0);
}
