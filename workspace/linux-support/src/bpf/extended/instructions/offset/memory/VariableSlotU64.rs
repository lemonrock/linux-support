// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A 64-bit sized slot suitable for a variable on the eBPF stack.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VariableSlotU64(u8);

impl TryFrom<u8> for VariableSlotU64
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if unlikely!((value as usize) > ((MAX_BPF_STACK as usize) / size_of::<u64>()))
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Into<RegisterOrImmediate<'static>> for VariableSlotU64
{
	#[inline(always)]
	fn into(self) -> RegisterOrImmediate<'static>
	{
		let offset = MemoryOffset::stack_variable_offset::<u64>(self.0 as i16) as i32;
		RegisterOrImmediate::Immediate(Immediate(Offset::Known(offset)))
	}
}

impl Into<MemoryOffset<'static>> for VariableSlotU64
{
	#[inline(always)]
	fn into(self) -> MemoryOffset<'static>
	{
		self.to_memory_offset()
	}
}

impl VariableSlotU64
{
	#[inline(always)]
	pub(crate) fn to_memory_slot_from_try_into(variable_slot: impl TryInto<VariableSlotU64>) -> MemoryOffset<'static>
	{
		let variable_slot = variable_slot.try_into().expect("Invalid variable slot u64");
		variable_slot.to_memory_offset()
	}
	
	#[inline(always)]
	pub(crate) const fn to_memory_offset(self) -> MemoryOffset<'static>
	{
		MemoryOffset::stack_variable_64(self.0)
	}
}
