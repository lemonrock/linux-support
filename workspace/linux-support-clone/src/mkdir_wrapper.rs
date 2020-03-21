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
		panic!("Invalid result from mkdir of `{}`", result)
	}
}
