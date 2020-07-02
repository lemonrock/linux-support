// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An immediate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Immediate<'de, IV: ImmediateValue>(pub Offset<'de, IV>);

impl<'de, IV: ImmediateValue> AsRef<Offset<'de, IV>> for Immediate<'de, IV>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'de, IV>
	{
		&self.0
	}
}

impl<'de, IV: ImmediateValue, V: Into<Offset<'de, IV>>> From<V> for Immediate<'de, IV>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}
