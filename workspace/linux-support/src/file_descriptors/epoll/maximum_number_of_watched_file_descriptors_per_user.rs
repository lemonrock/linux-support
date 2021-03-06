// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gets the maximum number of watched file descriptors per user.
///
/// Reads from `/proc/sys/fs/epoll/max_user_watches`.
///
/// Default varies; might be 204,328 on a system with 1Gb.
#[inline(always)]
pub fn maximum_number_of_watched_file_descriptors_per_user(proc_path: &ProcPath) -> NonZeroU32
{
	proc_path.sys_fs_epoll_file_path("max_user_watches").read_value().unwrap()
}
