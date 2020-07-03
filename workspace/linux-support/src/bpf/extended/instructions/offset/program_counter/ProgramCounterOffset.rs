// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A program counter (pc) offset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProgramCounterOffset<'name, PCOV: ProgramCounterOffsetValue>(#[serde(borrow)] pub Offset<'name, PCOV>);

impl<'name, PCOV: ProgramCounterOffsetValue> AsRef<Offset<'name, PCOV>> for ProgramCounterOffset<'name, PCOV>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'name, PCOV>
	{
		&self.0
	}
}

impl<'name, PCOV: ProgramCounterOffsetValue, V: Into<Offset<'name, PCOV>>> From<V> for ProgramCounterOffset<'name, PCOV>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}
