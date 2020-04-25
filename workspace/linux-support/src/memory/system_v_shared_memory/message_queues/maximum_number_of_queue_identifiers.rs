// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Default is 32,000.
///
/// Reads from `/proc/sys/kernel/msgmni`.
#[inline(always)]
pub fn maximum_number_of_queue_identifiers(proc_path: &ProcPath) -> NonZeroU32
{
	proc_path.sys_kernel_file_path("msgmni").read_value().unwrap()
}
