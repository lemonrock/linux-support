// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An immediate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Immediate<'name, IV: ImmediateValue>(#[serde(borrow)] pub Offset<'name, IV>);

impl<'name, IV: ImmediateValue> AsRef<Offset<'name, IV>> for Immediate<'name, IV>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'name, IV>
	{
		&self.0
	}
}

impl<'name, IV: ImmediateValue, V: Into<Offset<'name, IV>>> From<V> for Immediate<'name, IV>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}
