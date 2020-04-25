// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The `flags` argument is provided to allow for future extensions to the interface; in the current implementation it must be specified as `0`.
#[inline(always)]
pub(super) fn sched_getattr(pid: pid_t, attr: *mut sched_attr, size: u32, flags: c_uint) -> c_int
{
	SYS::sched_getattr.syscall4(pid as usize, attr as usize, size as usize, flags as usize) as c_int
}
