// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Prior to Linux 3.14 could not exceed 1,024 (`HARD_QUEUESMAX`).
///
/// Default is 256.
///
/// Writes to `/proc/sys/fs/mqueue/queues_max`.
#[inline(always)]
pub fn set_maximum_number_of_queues(proc_path: &ProcPath, maximum_number_of_queues: NonZeroU32) -> io::Result<()>
{
	proc_path.sys_fs_mqueue_file_path("queues_max").write_value(maximum_number_of_queues)
}
