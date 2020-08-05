// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn set_proc_sys_kernel_value<'a, E: error::Error>(proc_path: &ProcPath, file_name: &'static str, value: Option<impl IntoLineFeedTerminatedByteString<'a>>, error: impl FnOnce(io::Error) -> E) -> Result<(), E>
{
	set_proc_value(ProcPath::sys_kernel_file_path, proc_path, file_name, value, error)
}
