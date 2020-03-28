// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `nodemask` can be `null()`; in which case, `maxnode` is `0`.
///
/// See <http://man7.org/linux/man-pages/man2/get_mempolicy.2.html>.
#[inline(always)]
pub(super) fn get_mempolicy(mode: *mut i32, nodemask: *mut usize, maxnode: usize, addr: *const c_void, flags: GetMemoryPolicyFlags) -> isize
{
	SYS::get_mempolicy.syscall5(mode as usize, nodemask as usize, maxnode, addr as usize, flags.bits() as usize)
}
