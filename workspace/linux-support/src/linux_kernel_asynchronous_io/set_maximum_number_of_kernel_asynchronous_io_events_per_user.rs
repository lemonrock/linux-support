// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sets the maximum number of kernel asynchronous IO (KAIO) events per user.
///
/// This value is used as a limit to `nr_events` in `io_setup()`.
///
/// Writes to `/proc/sys/fs/aio-max-nr`.
///
/// Default is 65,536.
///
/// A typical value for performance might be 1,048,576.
#[inline(always)]
pub fn set_maximum_number_of_kernel_asynchronous_io_events_per_user(proc_path: &ProcPath, maximum_number_of_kernel_asynchronous_io_events_per_user: NonZeroU32) -> io::Result<()>
{
	proc_path.sys_fs_file_path("aio-max-nr").write_value(maximum_number_of_kernel_asynchronous_io_events_per_user)
}
