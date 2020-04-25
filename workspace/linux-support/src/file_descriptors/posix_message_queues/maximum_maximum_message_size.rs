// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Does not exceed 16,777,216 (`HARD_MSGSIZEMAX`).
///
/// Default is 8,192.
/// Minimum is 128.
///
/// Reads from `/proc/sys/fs/mqueue/msgsize_max`.
///
/// Revised in Linux 3.5.
#[inline(always)]
pub fn maximum_maximum_message_size(proc_path: &ProcPath) -> NonZeroU32
{
	proc_path.sys_fs_mqueue_file_path("msgsize_max").read_value().unwrap()
}
