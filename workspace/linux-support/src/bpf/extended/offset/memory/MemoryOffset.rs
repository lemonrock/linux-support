// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MemoryOffset<'de>(pub Offset<'de, i16>);

impl<'de> AsRef<Offset<'de>> for MemoryOffset<'de, i16>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'de, i16>
	{
		&self.0
	}
}

impl<'de, V: Into<Offset<'de, i16>>> From<V> for MemoryOffset<'de>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}
