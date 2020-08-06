// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Default is 18446744073692774399.
///
/// Writes to `/proc/sys/kernel/shmall`.
#[inline(always)]
pub fn set_maximum_number_of_shared_memory_pages(proc_path: &ProcPath, number_of_shared_memory_pages: NonZeroNumberOfPages) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/kernel/shmall");

	let file_path = proc_path.sys_kernel_file_path("shmall");
	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(number_of_shared_memory_pages))
	}
	else
	{
		Ok(())
	}
}
