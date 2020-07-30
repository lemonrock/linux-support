// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramCounter(pub(crate) usize);

impl ProgramCounter
{
	#[inline(always)]
	pub(crate) fn i16_offset(self, from_program_counter: Self) -> Result<i16, ParseError>
	{
		self.offset_to_label(from_program_counter, ParseError::JumpOffsetIsTooLargeForI16, i16::MAX as u64, (-(i16::MIN as i64)) as u64).map(|i32_offset| i32_offset as i16)
	}
	
	#[inline(always)]
	pub(crate) fn i32_offset(self, from_program_counter: Self) -> Result<i32, ParseError>
	{
		self.offset_to_label(from_program_counter, ParseError::RelativeFunctionOffsetIsTooLargeForI32, i32::MAX as u64, (-(i32::MIN as i64)) as u64)
	}
	
	/// Unconditional jumps simply move the program counter forward, so that the next instruction to be executed relative to the current instruction is `off + 1`, where `off` is the constant offset encoded in the instruction.
	/// If the condition in a conditional jump operation results in true, then a relative jump to `off + 1` is performed, otherwise the next instruction `0 + 1` is performed.
	/// Hence an `off` of `-1` will always cause a jump to the jump instruction itself, which must be illegal.
	#[inline(always)]
	fn offset_to_label(self, from_program_counter: Self, error_if_too_large: ParseError, maximum: u64, minimum: u64) -> Result<i32, ParseError>
	{
		let label = self.0 as u64;
		let from_program_counter = from_program_counter.0 as u64;
		if label > from_program_counter
		{
			let difference = label - from_program_counter - 1;
			if unlikely!(difference > maximum)
			{
				Err(error_if_too_large)
			}
			else
			{
				Ok(difference as i32)
			}
		}
		else if label < from_program_counter
		{
			match (from_program_counter - label).checked_add(1)
			{
				None => Err(error_if_too_large),
				
				Some(difference) =>
				{
					if unlikely!(difference > minimum)
					{
						Err(error_if_too_large)
					}
					else
					{
						Ok(-(difference as i64) as i32)
					}
				}
			}
		}
		else
		{
			Err(ParseError::JumpOrRelativeFunctionOffsetOfNegativeOneCreatesAnInfiniteLoop)
		}
	}
}
