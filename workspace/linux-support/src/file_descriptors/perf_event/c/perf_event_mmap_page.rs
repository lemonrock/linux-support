// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct perf_event_mmap_page
{
	pub(crate) version: u32,
	pub(crate) compat_version: u32,
	pub(crate) lock: u32,
	pub(crate) index: u32,
	pub(crate) offset: i64,
	pub(crate) time_enabled: u64,
	pub(crate) time_running: u64,
	pub(crate) __bindgen_anon_1: perf_event_mmap_page__bindgen_ty_1,
	pub(crate) pmc_width: u16,
	pub(crate) time_shift: u16,
	pub(crate) time_mult: u32,
	pub(crate) time_offset: u64,
	pub(crate) time_zero: u64,
	pub(crate) size: u32,
	pub(crate) __reserved: [u8; 948],
	pub(crate) data_head: u64,
	pub(crate) data_tail: u64,
	pub(crate) data_offset: u64,
	pub(crate) data_size: u64,
	pub(crate) aux_head: u64,
	pub(crate) aux_tail: u64,
	pub(crate) aux_offset: u64,
	pub(crate) aux_size: u64,
}

impl Default for perf_event_mmap_page
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for perf_event_mmap_page
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "perf_event_mmap_page {{ version: {:?}, compat_version: {:?}, lock: {:?}, index: {:?}, offset: {:?}, time_enabled: {:?}, time_running: {:?}, __bindgen_anon_1: {:?}, pmc_width: {:?}, time_shift: {:?}, time_mult: {:?}, time_offset: {:?}, time_zero: {:?}, size: {:?}, __reserved: [{}], data_head: {:?}, data_tail: {:?}, data_offset: {:?}, data_size: {:?}, aux_head: {:?}, aux_tail: {:?}, aux_offset: {:?}, aux_size: {:?} }}", self.version, self.compat_version, self.lock, self.index, self.offset, self.time_enabled, self.time_running, self.__bindgen_anon_1, self.pmc_width, self.time_shift, self.time_mult, self.time_offset, self.time_zero, self.size, self.__reserved.iter().enumerate().map(|(i, v)| format!("{}{:?}", if i > 0 { ", " } else { "" }, v)).collect::<String>(), self.data_head, self.data_tail, self.data_offset, self.data_size, self.aux_head, self.aux_tail, self.aux_offset, self.aux_size )
	}
}
