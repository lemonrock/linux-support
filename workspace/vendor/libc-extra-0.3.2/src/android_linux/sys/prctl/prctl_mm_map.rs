// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct prctl_mm_map
{
	pub start_code: uint64_t,
	pub end_code: uint64_t,
	pub start_data: uint64_t,
	pub end_data: uint64_t,
	pub start_brk: uint64_t,
	pub brk: uint64_t,
	pub start_stack: uint64_t,
	pub arg_start: uint64_t,
	pub arg_end: uint64_t,
	pub env_start: uint64_t,
	pub env_end: uint64_t,
	pub auxv: *mut uint64_t,
	pub auxv_size: uint32_t,
	pub exe_fd: uint32_t,
}
