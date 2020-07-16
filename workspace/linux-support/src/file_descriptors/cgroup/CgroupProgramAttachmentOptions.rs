// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// How to attach the program.
///
/// This is to support a complex hierarchy of Cgroups.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CgroupProgramAttachmentOptions
{
	/// This is the only program at this level; no child programs are allowed.
	AddLeaf,

	/// This is the only program at this level, but child programs are allowed.
	AddButOverridable,

	/// One or more child programs are allowed at this level.
	AddMultiple,

	/// Replace a program with this program.
	Replace(FileDescriptorCopy<ExtendedBpfProgramFileDescriptor>),
}

impl ProgramAttachmentOptions for CgroupProgramAttachmentOptions
{
	#[inline(always)]
	fn to_attach_flags(self) -> (BPF_PROG_ATTACH_flags, RawFd)
	{
		use self::CgroupProgramAttachmentOptions::*;
		
		match self
		{
			AddLeaf => (BPF_PROG_ATTACH_flags::empty(), 0),
			
			AddButOverridable => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_OVERRIDE, 0),
			
			AddMultiple => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_MULTI, 0),
			
			Replace(replace) => (BPF_PROG_ATTACH_flags::BPF_F_ALLOW_MULTI, replace.as_raw_fd()),
		}
	}
}
