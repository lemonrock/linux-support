// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A program counter (pc) offset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProgramCounterOffset<'de, PCOV: ProgramCounterOffsetValue>(pub Offset<'de, PCOV>);

impl<'de, PCOV: ProgramCounterOffsetValue> AsRef<Offset<'de, PCOV>> for ProgramCounterOffset<'de, PCOV>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'de, PCOV>
	{
		&self.0
	}
}

impl<'de, PCOV: ProgramCounterOffsetValue, V: Into<Offset<'de, PCOV>>> From<V> for Immediate<'de, PCOV>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}
