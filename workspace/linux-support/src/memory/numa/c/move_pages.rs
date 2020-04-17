// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(super) fn move_pages(pid: pid_t, count: usize, pages: *const *const c_void, nodes: *const i32, status: *mut i32, flags: MemoryBindFlags) -> isize
{
	SYS::move_pages.syscall6(pid as usize, count, pages as usize, nodes as usize, status as usize, flags.bits as usize)
}
