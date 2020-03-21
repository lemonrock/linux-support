#[inline(always)]
fn setresgid_wrapper(real_group_identifier: gid_t, effective_group_identifier: gid_t, saved_set_group_identifier: gid_t)
{
	let result = unsafe { setresgid(real_group_identifier, effective_group_identifier, saved_set_group_identifier) };
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
