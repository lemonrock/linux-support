// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(unused_variables)]
#[inline(always)]
pub(super) fn openat2(dirfd: c_int, pathname: *const c_char, how: *mut open_how, size: size_t) -> c_int
{
	unimplemented!("Not yet implemented by musl and syscall number is unknown")
}
