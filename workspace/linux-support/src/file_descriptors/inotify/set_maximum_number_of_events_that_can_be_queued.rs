// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sets the maximum number of events that can be queued.
///
/// Writes to `/proc/sys/fs/inotify/max_queued_events`.
///
/// Default is 16,384.
#[inline(always)]
pub fn set_maximum_number_of_events_that_can_be_queued(proc_path: &ProcPath, maximum_number_of_events_that_can_be_queued: NonZeroU32) -> io::Result<()>
{
	proc_path.sys_fs_inotify_file_path("max_queued_events").write_value(maximum_number_of_events_that_can_be_queued)
}
