#[inline(always)]
fn setresuid_wrapper(real_user_identifier: uid_t, effective_user_identifier: uid_t, saved_set_user_identifier: uid_t)
{
	let result = unsafe { setresuid(real_user_identifier, effective_user_identifier, saved_set_user_identifier) };
	if likely!(result == 0)
	{
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EAGAIN => panic!("uid does not match the current UID and this call would bring that user ID over its `RLIMIT_NPROC` resource limit"),
			EPERM => panic!("The calling process is not privileged (did not have the `CAP_SETUID` capability) and tried to change the IDs to values that are not permitted."),

			unknown @ _ => panic!("Unknown error `{}` from `setresuid()`", unknown),
		}
	}
	else
	{
		panic!("Invalid result `{}` from setresuid()", result)
	}
}
