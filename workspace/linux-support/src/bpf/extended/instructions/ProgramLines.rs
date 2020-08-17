// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Program lines.
///
/// There must be at least one line.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProgramLines<'name>(#[serde(borrow)] Vec<ProgramLine<'name>>);

impl<'name> Deref for ProgramLines<'name>
{
	type Target = Vec<ProgramLine<'name>>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<'name> DerefMut for ProgramLines<'name>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<'name> ProgramLines<'name>
{
	/// Adds a block of lines.
	#[inline(always)]
	pub fn new(program_line_wrappers: Vec<ProgramLineWrapper<'name>>) -> Self
	{
		let mut this = Self(Vec::with_capacity(program_line_wrappers.len() * 2));
		
		this.extend(program_line_wrappers);
		
		this.shrink_to_fit();
		this
	}
	
	#[inline(always)]
	fn extend(&mut self, program_line_wrappers: Vec<ProgramLineWrapper<'name>>)
	{
		assert_ne!(program_line_wrappers.len(), 0, "There must be at least one program line");
		for wrapper in program_line_wrappers
		{
			wrapper.extend(self)
		}
		
	}
}
