// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(unused_variables)]
#[inline(always)]
pub(super) fn openat2(dirfd: c_int, pathname: *const c_char, how: *mut open_how, size: size_t) -> isize
{
	SYS::openat2.syscall4(dirfd as usize, pathname as usize, how as usize, size as usize)
}
