/// Converts a C string pointer.
#[inline(always)]
pub fn c_string_pointer_to_path_buf(nul_terminated: *mut c_char) -> Result<Option<PathBuf>, ()>
{
	if unlikely!(nul_terminated.is_null())
	{
		return Ok(None);
	}

	let c_str = unsafe { CStr::from_ptr(nul_terminated) };

	let bytes = c_str.to_bytes();
	if bytes.len() == 0
	{
		Err(())
	}
	else
	{
		let os_str: &OsStr = OsStrExt::from_bytes(bytes);
		Ok(Some(PathBuf::from(os_str)))
	}
}