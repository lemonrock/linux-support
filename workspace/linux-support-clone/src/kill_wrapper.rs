#[inline(always)]
fn kill_wrapper(child_process_identifier: NonZeroU32)
{
	let result = unsafe { kill(child_process_identifier.get().try_into().unwrap(), SIGKILL) };
	if likely!(result == 0)
	{
		return
	}
	else if likely!(result == -1)
	{
		match errno().0
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
