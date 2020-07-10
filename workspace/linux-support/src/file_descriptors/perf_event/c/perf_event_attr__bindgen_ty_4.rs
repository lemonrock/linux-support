// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union perf_event_attr__bindgen_ty_4
{
	pub(crate) bp_len: u64,
	pub(crate) kprobe_addr: u64,
	pub(crate) probe_offset: u64,
	pub(crate) config2: u64,
	_bindgen_union_align: u64,
}

impl Default for perf_event_attr__bindgen_ty_4
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for perf_event_attr__bindgen_ty_4
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_attr__bindgen_ty_4 {{ union }}")
	}
}
