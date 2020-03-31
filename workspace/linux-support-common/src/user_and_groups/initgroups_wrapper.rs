// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Requires `/etc/group` to exist.
#[inline(always)]
fn initgroups_wrapper(effective_user_name: &CStr, effective_group_identifier: GroupIdentifier)
{
	// Uses `getgrouplist()` and `setgroups()` under the covers.
	let result = unsafe { initgroups(effective_user_name.as_ptr(), effective_group_identifier.into()) };
	if likely!(result == 0)
	{
		return
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EFAULT => panic!("Invalid address for `list`"),
			EINVAL => panic!("size is less than the number of supplementary group IDs, but is not zero, or size is greater than `NGROUPS_MAX` (32 before Linux 2.6.4; 65536 since Linux 2.6.4"),
			ENOMEM => panic!("Out of memory"),
			EPERM => panic!("The calling process is not privileged (did not have the `CAP_SETGID` capability) and tried to change the IDs to values that are not permitted, or the use of `setgroups()` is denied in this user namespace. See the description of `/proc/[pid]/setgroups` in `user_namespaces(7)`."),

			// It looks likely that ENOENT and ENOTDIR are possible from reading the musl source code for `getgrouplist()`.
			unknown @ _ => panic!("Unknown error `{}` from `initgroups()`", unknown),
		}
	}
	else
	{
		panic!("Invalid result `{}` from setresgid()", result)
	}
}
