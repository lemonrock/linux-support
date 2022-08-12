// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn kill_wrapper(child_process_identifier: ProcessIdentifier)
{
	let result = unsafe { kill(child_process_identifier.into(), SIGKILL) };
	if likely!(result == 0)
	{
		return
	}
	else if likely!(result == -1)
	{
		match SystemCallErrorNumber::from_errno()
		{
			ESRCH => return,
			EINVAL => panic!("EINVAL from `kill()`"),
			EPERM => panic!("Permission defined"),

			unknown @ _ => panic!(("Unexpected error code of `{}` from `kill()`", unknown))
		}
	}
	else
	{
		panic!("Unexpected result code of `{}` from `kill()`", result)
	}
}
