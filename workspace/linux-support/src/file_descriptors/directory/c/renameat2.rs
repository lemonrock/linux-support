// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// int oldfd, const char *old, int newfd, const char *new
#[inline(always)]
pub(super) fn renameat2(oldfd: c_int, old: *const c_char, newfd: c_int, new: *const c_char, flags: c_int) -> c_int
{
	SYS::renameat2.syscall5(oldfd as usize, old as usize, newfd as usize, new as usize, flags as usize) as c_int
}
