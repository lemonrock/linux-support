// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A program counter (pc) offset value.
pub trait ProgramCounterOffsetValue: OffsetValue
{
	#[doc(hidden)]
	const Invalid: Self;
	
	#[doc(hidden)]
	fn program_counter_offset_value(label: ProgramCounter, current_program_counter: ProgramCounter) -> Result<Self, ParseError>;
}

impl ProgramCounterOffsetValue for i16
{
	const Invalid: Self = -1;
	
	#[inline(always)]
	fn program_counter_offset_value(label: ProgramCounter, current_program_counter: ProgramCounter) -> Result<Self, ParseError>
	{
		label.i16_offset(current_program_counter)
	}
}

impl ProgramCounterOffsetValue for i32
{
	const Invalid: Self = -1;
	
	#[inline(always)]
	fn program_counter_offset_value(label: ProgramCounter, current_program_counter: ProgramCounter) -> Result<Self, ParseError>
	{
		label.i32_offset(current_program_counter)
	}
}
