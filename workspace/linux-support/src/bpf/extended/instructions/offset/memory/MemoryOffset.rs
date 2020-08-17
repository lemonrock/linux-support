// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A memory offset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct MemoryOffset<'name>(#[serde(borrow)] pub Offset<'name, i16>);

impl<'name> AsRef<Offset<'name, i16>> for MemoryOffset<'name>
{
	#[inline(always)]
	fn as_ref(&self) -> &Offset<'name, i16>
	{
		&self.0
	}
}

impl<'name, V: Into<Offset<'name, i16>>> From<V> for MemoryOffset<'name>
{
	#[inline(always)]
	fn from(value: V) -> Self
	{
		Self(value.into())
	}
}

impl MemoryOffset<'static>
{
	/// Does not accommodate the upper-half of a BPF stack.
	#[inline(always)]
	pub(crate) fn stack_variable_8(variable_slot: u16) -> Self
	{
		assert!(variable_slot < (MAX_BPF_STACK as u16), "variable slot equals or exceeds maximum stack depth of MAX_BPF_STACK {}", MAX_BPF_STACK);
		Self::stack_variable::<u16>(variable_slot as i16)
	}
	
	#[inline(always)]
	pub(crate) const fn stack_variable_16(variable_slot: u8) -> Self
	{
		Self::stack_variable::<u16>(variable_slot as i16)
	}
	
	#[inline(always)]
	pub(crate) const fn stack_variable_32(variable_slot: u8) -> Self
	{
		Self::stack_variable::<u32>(variable_slot as i16)
	}
	
	#[inline(always)]
	pub(crate) const fn stack_variable_64(variable_slot: u8) -> Self
	{
		Self::stack_variable::<u64>(variable_slot as i16)
	}
	
	#[inline(always)]
	const fn stack_variable<V: Sized>(variable_slot: i16) -> Self
	{
		MemoryOffset::from(Offset::Known(Self::stack_variable_offset::<V>(variable_slot)))
	}
	
	#[inline(always)]
	const fn stack_variable_offset<V: Sized>(variable_slot: i16) -> i16
	{
		-(variable_slot * (size_of::<V>() as i16))
	}
}
