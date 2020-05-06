// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(super) fn io_uring_enter(fd: RawFd, to_submit: c_uint, min_complete: c_uint, flags: EnterFlags, sig: *mut sigset_t) -> c_int
{
	let flags = flags.bits;
	return SYS::io_uring_enter.syscall6(fd as usize, to_submit as usize, min_complete as usize, flags as usize, sig as usize, (_NSIG / 8) as usize) as i32;
}
