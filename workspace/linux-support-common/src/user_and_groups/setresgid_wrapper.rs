// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn setresgid_wrapper(real_group_identifier: GroupIdentifier, effective_group_identifier: GroupIdentifier, saved_set_group_identifier: GroupIdentifier)
{
	let result = unsafe { setresgid(real_group_identifier.into(), effective_group_identifier.into(), saved_set_group_identifier.into()) };
	if likely!(result == 0)
	{
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EAGAIN => panic!("uid does not match the current UID and this call would bring that user ID over its `RLIMIT_NPROC` resource limit"),
			EPERM => panic!("The calling process is not privileged (did not have the `CAP_SETGID` capability) and tried to change the IDs to values that are not permitted."),

			unknown @ _ => panic!("Unknown error `{}` from `setresgid()`", unknown),
		}
	}
	else
	{
		panic!("Invalid result `{}` from setresgid()", result)
	}
}
