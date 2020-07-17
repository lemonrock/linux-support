// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// BTF information.
#[repr(C, align(8))]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct bpf_btf_info
{
	pub(crate) btf: AlignedU64,
	
	pub(crate) btf_size: u32,
	
	pub(crate) id: BtfIdentifier,
}

impl Information for bpf_btf_info
{
	type Identifier = BtfIdentifier;
	
	#[inline(always)]
	fn identifier(&self) -> Self::Identifier
	{
		self.id
	}
}

impl bpf_btf_info
{
	/// BTF data, including header (`btf_header`) and sections.
	#[inline(always)]
	pub fn data(&self) -> Option<&[u8]>
	{
		self.btf.to_slice(self.btf_size)
	}
}
