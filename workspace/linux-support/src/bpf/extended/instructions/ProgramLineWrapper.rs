// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program line wrapper.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProgramLineWrapper<'name>
{
	/// Program Line.
	ProgramLine(ProgramLine<'name>),
	
	/// Program Lines.
	ProgramLines(Vec<ProgramLineWrapper<'name>>)
}

impl<'name> From<ProgramLine<'name>> for ProgramLineWrapper<'name>
{
	#[inline(always)]
	fn from(value: ProgramLine<'name>) -> Self
	{
		ProgramLineWrapper::ProgramLine(value)
	}
}

impl<'name> ProgramLineWrapper<'name>
{
	fn extend(self, program_lines: &mut ProgramLines<'name>)
	{
		use self::ProgramLineWrapper::*;
	
		match self
		{
			ProgramLine(program_line) => program_lines.push(program_line),
			
			ProgramLines(program_line_wrappers) => program_lines.extend(program_line_wrappers),
		}
	}
}
