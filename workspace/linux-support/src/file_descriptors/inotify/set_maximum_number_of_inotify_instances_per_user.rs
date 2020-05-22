// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sets the maximum number of inotify instances per user.
///
/// Writes to `/proc/sys/fs/inotify/max_user_instances`.
///
/// Default is 128.
#[inline(always)]
pub fn set_maximum_number_of_inotify_instances_per_user(proc_path: &ProcPath, maximum_number_of_inotify_instances_per_user: NonZeroU32) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/fs/inotify/max_user_instances");

	let file_path = proc_path.sys_fs_inotify_file_path("max_user_instances");
	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(maximum_number_of_inotify_instances_per_user))
	}
	else
	{
		Ok(())
	}
}
