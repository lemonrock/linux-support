// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Seccomp is disabled after this call.
#[inline(always)]
pub fn disabled_seccomp() -> io::Result<()>
{
	let result = unsafe { prctl(PR_SET_SECCOMP, SECCOMP_MODE_DISABLED, 0, 0, 0) };
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
		unreachable!("prctl() returned unexpected result {}", result)
	}
}
