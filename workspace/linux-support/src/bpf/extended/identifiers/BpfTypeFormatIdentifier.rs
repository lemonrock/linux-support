// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Identifier for BPF Type Format (BTF) data.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BpfTypeFormatIdentifier(u32);

impl From<u32> for BpfTypeFormatIdentifier
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		Self(value)
	}
}

impl Into<u32> for BpfTypeFormatIdentifier
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0
	}
}

impl Into<BpfCommandGetIdentifierValueOfIdentifier> for BpfTypeFormatIdentifier
{
	#[inline(always)]
	fn into(self) -> BpfCommandGetIdentifierValueOfIdentifier
	{
		BpfCommandGetIdentifierValueOfIdentifier
		{
			btf_id: self.0
		}
	}
}

impl Identifier for BpfTypeFormatIdentifier
{
	const Next: bpf_cmd = bpf_cmd::BPF_BTF_GET_NEXT_ID;
}
