// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Does not exceed ?
///
/// Default is 18446744073692774399.
///
/// Reads from `/proc/sys/kernel/shmmax`.
#[inline(always)]
pub fn maximum_memory_segment_size(proc_path: &ProcPath) -> NonZeroU64
{
	proc_path.sys_kernel_file_path("shmmax").read_value().unwrap()
}
