// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A register or immediate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum RegisterOrImmediate<'name>
{
	/// Register.
	Register(Register),

	/// Immediate.
	Immediate(#[serde(borrow)] Immediate<'name, i32>)
}

impl<'name> From<Register> for RegisterOrImmediate<'name>
{
	#[inline(always)]
	fn from(value: Register) -> Self
	{
		RegisterOrImmediate::Register(value)
	}
}

impl<'name, V: Into<Immediate<'name, i32>>> From<V> for RegisterOrImmediate<'name>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		RegisterOrImmediate::Immediate(value.into())
	}
}
