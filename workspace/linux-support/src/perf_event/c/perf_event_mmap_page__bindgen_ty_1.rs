// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union perf_event_mmap_page__bindgen_ty_1
{
	pub(crate) capabilities: u64,
	pub(crate) __bindgen_anon_1: perf_event_mmap_page__bindgen_ty_1__bindgen_ty_1,
}

impl Default for perf_event_mmap_page__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe_zeroed()
	}
}

impl Debug for perf_event_mmap_page__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_mmap_page__bindgen_ty_1 {{ union }}")
	}
}
