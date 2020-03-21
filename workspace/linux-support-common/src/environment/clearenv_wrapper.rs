#[inline(always)]
fn clearenv_wrapper()
{
	let result = unsafe { clearenv() };
	if likely!(result == 0)
	{
	}
	else if unlikely!(result == -1)
	{
		panic!("`clearenv()` failed")
	}
	else
	{
		panic!("`clearenv()` failed with an unexpecte result `{}`", result)
	}
}
