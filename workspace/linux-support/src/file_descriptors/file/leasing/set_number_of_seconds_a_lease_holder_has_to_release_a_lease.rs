// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Sets the current grace time in seconds in `/proc/sys/fs/lease-break-time`.
///
/// Default is 45 seconds.
#[inline(always)]
pub fn set_number_of_seconds_a_lease_holder_has_to_release_a_lease(proc_path: &ProcPath, number_of_seconds: usize) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/fs/lease-break-time");

	let file_path = proc_path.sys_fs_file_path("lease-break-time");
	if file_path.exists()
	{
		file_path.write_value(number_of_seconds)
	}
	else
	{
		Ok(())
	}
}
