// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn perf_event_open(perf_event_attr: &mut attr, pid: pid_t, cpu: c_int, group_fd: RawFd, flags: c_ulong) -> i32
{
	SYS::perf_event_open.syscall5(perf_event_attr as *mut attr as usize, pid as usize, cpu as usize, group_fd as usize, flags as usize) as i32
}
