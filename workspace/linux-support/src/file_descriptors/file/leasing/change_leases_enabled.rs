// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Change whether leases enabled?
///
/// Writes to `/proc/sys/fs/leases-enable`.
///
/// Default is `true`
#[inline(always)]
pub fn change_leases_enabled(proc_path: &ProcPath, enable_if_true_disable_if_false: bool) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/fs/leases-enable");

	let file_path = proc_path.sys_fs_file_path("leases-enable");
	if file_path.exists()
	{
		file_path.write_value(enable_if_true_disable_if_false)
	}
	else
	{
		Ok(())
	}
}
