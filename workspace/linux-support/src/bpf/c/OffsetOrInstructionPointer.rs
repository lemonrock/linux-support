// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union OffsetOrInstructionPointer
{
	/// Offset.
	pub(crate) offset: u64,
	
	/// Instruction pointer.
	pub(crate) ip: InstructionPointer,
}

impl Default for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "OffsetOrInstructionProtocol {{ {} }}", unsafe { self.offset })
	}
}

impl PartialEq for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		unsafe { self.offset == rhs.offset }
	}
}

impl Eq for OffsetOrInstructionPointer
{
}

impl PartialOrd for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		unsafe { self.offset.partial_cmp(&rhs.offset) }
	}
}

impl Ord for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		unsafe { self.offset.cmp(&rhs.offset) }
	}
}

impl Hash for OffsetOrInstructionPointer
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.offset.hash(hasher) }
	}
}
