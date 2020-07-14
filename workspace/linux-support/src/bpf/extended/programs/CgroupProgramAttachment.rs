// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How to attach the program.
///
/// Mostly this is to support a complex hierarchy of Cgroups.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CgroupProgramAttachment<'a>
{
	/// This is the only program at this level; no child programs are allowed.
	AddLeaf,

	/// This is the only program at this level, but child programs are allowed.
	AddButOverridable,

	/// One or more child programs are allowed at this level.
	AddMultiple,

	/// Replace a program with this program.
	Replace(&'a ExtendedBpfProgramFileDescriptor)
}

impl<'a> CgroupProgramAttachment<'a>
{
	#[inline(always)]
	pub(crate) fn to_attach_flags(self) -> (BPF_PROG_ATTACH_flags, Option<NonZeroI32>)
	{
		use self::CgroupProgramAttachment::*;
		
		match self
		{
			AddLeaf => (BPF_PROG_ATTACH_flags::empty(), None),
			
			AddButOverridable => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_OVERRIDE, None),
			
			AddMultiple => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_MULTI, None),
			
			Replace(replace) => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_MULTI, Some(replace.as_non_zero_i32())),
		}
	}
}
