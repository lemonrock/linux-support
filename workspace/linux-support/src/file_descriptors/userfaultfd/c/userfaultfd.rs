// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `flags` is a combination of `O_CLOEXEC` (`UFFD_CLOEXEC`), `O_NONBLOCK` (`UFFD_NONBLOCK`) and `UFFD_USER_MODE_ONLY`.
/// `UFFD_SHARED_FCNTL_FLAGS`, used internally, is `UFFD_CLOEXEC | UFFD_NONBLOCK`.
#[inline(always)]
pub(super) fn userfaultfd(flags: i32) -> i32
{
	SYS::userfaultfd.syscall1(flags as usize) as i32
}
