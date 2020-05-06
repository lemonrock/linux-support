// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
pub(super) fn ioprio_set(which: c_int, who: IOPRIO_WHO, ioprio: u16) -> c_int
{
	let who = who as i32;
	let ioprio = ioprio as i32;
	SYS::ioprio_set.syscall3(which as usize, who as usize, ioprio as usize) as i32
}
