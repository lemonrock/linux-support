// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `fd` is the file descriptor returned by a call to `io_uring_setup()`.
/// `args` is a list with `nr_args` elements.
/// The type of `args` depends on the value of `opcode`.
#[inline(always)]
pub(super) fn io_uring_register(fd: RawFd, opcode: RegisterOperation, arg: *mut c_void, nr_args: c_uint) -> c_int
{
	let opcode = opcode as u32;
	return SYS::io_uring_register.syscall4(fd as usize, opcode as usize, arg as usize, nr_args as usize) as i32;
}
