// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Prior to Linux 3.14 could not exceed 1,024 (`HARD_QUEUESMAX`).
///
/// Default is 256.
///
/// Reads from `/proc/sys/fs/mqueue/queues_max`.
#[inline(always)]
pub fn maximum_number_of_queues(proc_path: &ProcPath) -> NonZeroU32
{
	proc_path.sys_mqueue_file_path("queues_max").read_value().unwrap()
}
