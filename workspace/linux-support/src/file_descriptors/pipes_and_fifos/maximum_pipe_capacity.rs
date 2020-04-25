// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Value of `/proc/sys/fs/pipe-max-size`.
///
/// The default is 1Mb.
///
/// It is never less than the system's page size.
#[inline(always)]
pub fn maximum_pipe_capacity(proc_path: &ProcPath) -> NonZeroU32
{
	proc_path.sys_fs_file_path("pipe-max-size").read_value().unwrap()
}
