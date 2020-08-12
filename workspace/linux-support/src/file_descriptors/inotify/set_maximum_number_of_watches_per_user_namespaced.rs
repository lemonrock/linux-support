// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sets the maximum number of watches per user.
///
/// Writes to `/proc/sys/user/max_inotify_watches`.
///
/// Default is 8,192.
#[inline(always)]
pub fn set_maximum_number_of_watches_per_user_namespaced(proc_path: &ProcPath, maximum_number_of_watches_per_user: NonZeroU32) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/user/max_inotify_watches");

	let file_path = proc_path.sys_user_file_path("max_inotify_watches");
	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(maximum_number_of_watches_per_user))
	}
	else
	{
		Ok(())
	}
}
