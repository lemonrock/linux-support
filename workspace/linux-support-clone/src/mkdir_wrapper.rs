// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn mkdir_wrapper(path: &Path, mode: mode_t) -> io::Result<()>
{
	let path = path_to_cstring(path);
	let result = unsafe { mkdir(path.as_ptr() as *const c_char, mode) };
	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		Err(io::Error::last_os_error())
	}
	else
	{
		unexpected_result!(mkdir, result)
	}
}
