// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Gets the current grace time in seconds from `/proc/sys/fs/lease-break-time`.
///
/// Default is 45 seconds.
#[inline(always)]
pub fn number_of_seconds_a_lease_holder_has_to_release_a_lease(proc_path: &ProcPath) -> usize
{
	proc_path.sys_fs_file_path("lease-break-time").read_value().unwrap()
}
