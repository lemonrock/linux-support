// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Does not exceed 65,536 (`HARD_MSGMAX`).
///
/// Default is 10.
///
/// Writes to `/proc/sys/fs/mqueue/msg_max`.
///
/// Since Linux 3.5.
/// Revised in Linux 3.5.
#[inline(always)]
pub fn set_maximum_maximum_number_of_messages_in_a_queue(proc_path: &ProcPath, maximum_maximum_number_of_messages_in_a_queue: NonZeroU32) -> io::Result<()>
{
	debug_assert!(value.get() <= 65536);

	proc_path.sys_fs_mqueue_file_path("msg_max").write_value(maximum_maximum_number_of_messages_in_a_queue)
}
