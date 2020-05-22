// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Set the soft limit.
///
/// The soft limit is the total size (in pages) of all pipes created or set by a single unprivileged user (one without the `CAP_SYS_RESOURCE` and `CAP_SYS_ADMIN` capabilities), after which an user can only create pipes with a size of one page.
///
/// Writes to `/proc/sys/fs/pipe-user-pages-soft`.
///
/// `None` means no limit.
/// Default is 16,384.
///
/// Since Linux 4.5.
/// Not buggy since Linux 4.9.
#[inline(always)]
pub fn set_pipe_user_pages_soft_limit(proc_path: &ProcPath, soft_limit: Option<NonZeroNumberOfPages>) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/fs/pipe-user-pages-soft");
	let file_path = proc_path.sys_fs_file_path("pipe-user-pages-soft");

	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(soft_limit))
	}
	else
	{
		Ok(())
	}
}
