// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Value of `/proc/sys/fs/pipe-max-size`.
///
/// The default is 1Mb.
///
/// `maximum_capacity` can not be less than the system's page size.
#[inline(always)]
pub fn set_maximum_pipe_capacity(proc_path: &ProcPath, maximum_pipe_capacity: NonZeroU32) -> io::Result<()>
{
	debug_assert!(PageSize::default().size_in_bytes().get() as u32 <= maximum_pipe_capacity.get(), "maximum_pipe_capacity {} can not be less than system page size", maximum_pipe_capacity);

	assert_effective_user_id_is_root("Change /proc/sys/fs/pipe-max-size");

	let file_path = proc_path.sys_fs_file_path("pipe-max-size");
	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(maximum_pipe_capacity))
	}
	else
	{
		Ok(())
	}
}
