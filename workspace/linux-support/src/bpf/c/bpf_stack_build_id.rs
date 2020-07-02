// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct bpf_stack_build_id
{
	pub(crate) status: bpf_stack_build_id_status,
	pub(crate) build_id: [c_uchar; Self::BPF_BUILD_ID_SIZE],
	pub(crate) offset_or_internet_protocol: OffsetOrInternetProtocol,
}

impl Default for bpf_stack_build_id
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for bpf_stack_build_id
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "bpf_stack_build_id {{ status: {:?}, build_id: {:?}, offset_or_internet_protocol: {:?} }}", self.status, self.build_id, self.offset_or_internet_protocol)
	}
}

impl bpf_stack_build_id
{
	pub(crate) const BPF_BUILD_ID_SIZE: usize = 20;
}