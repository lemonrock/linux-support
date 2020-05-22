// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Default is 16,384.
///
/// Writes to `/proc/sys/kernel/msgmni`.
#[inline(always)]
pub fn set_maximum_queue_size_in_bytes(proc_path: &ProcPath, maximum_queue_size_in_bytes: NonZeroU32) -> io::Result<()>
{
	assert_effective_user_id_is_root("write /proc/sys/kernel/msgmni");

	let file_path = proc_path.sys_kernel_file_path("msgmni");
	if file_path.exists()
	{
		file_path.write_value(UnpaddedDecimalInteger(maximum_queue_size_in_bytes))
	}
	else
	{
		Ok(())
	}
}
