// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gets the current number of kernel asynchronous IO events (KAIO).
///
/// Reads from `/proc/sys/fs/aio-nr`.
///
/// On a quiet system this is 0.
#[inline(always)]
pub fn current_number_of_kernel_asynchronous_io_events_per_user(proc_path: &ProcPath) -> u32
{
	proc_path.sys_fs_file_path("aio-nr").read_value().unwrap()
}
