// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Identifier for an eBPF program.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtendedBpfProgramIdentifier(u32);

impl From<u32> for ExtendedBpfProgramIdentifier
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Self(value)
	}
}

impl Into<u32> for ExtendedBpfProgramIdentifier
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl Into<BpfCommandGetIdentifierValueOfIdentifier> for ExtendedBpfProgramIdentifier
{
	#[inline(always)]
	fn into(self) -> BpfCommandGetIdentifierValueOfIdentifier
	{
		BpfCommandGetIdentifierValueOfIdentifier
		{
			prog_id: self.0
		}
	}
}

impl Identifier for ExtendedBpfProgramIdentifier
{
	const Next: bpf_cmd = bpf_cmd::BPF_PROG_GET_NEXT_ID;
	
	const GetFileDescriptor: bpf_cmd = bpf_cmd::BPF_PROG_GET_FD_BY_ID;
	
	type FD = ExtendedBpfProgramFileDescriptor;
	
	type Access = ();
	
	#[inline(always)]
	fn access_permissions_to_open_flags(_access: Self::Access) -> u32
	{
		0
	}
}
