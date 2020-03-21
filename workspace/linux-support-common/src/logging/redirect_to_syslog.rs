#[inline(always)]
fn redirect_to_syslog(original: &mut *const FILE, callback: cookie_write_function_t)
{
	const w: ConstCStr = ConstCStr(b"w\0");

	let mut functions = cookie_io_functions_t::default();
	functions.write = callback;

	let file = unsafe { fopencookie(null_mut(), w.as_ptr(), functions) };
	assert!(!file.is_null(), "file is null from `fopencookie()`");
	*original = file;
	let result = unsafe { setvbuf(*original as *mut _, null_mut(), _IOLBF, 0) };
	assert_eq!(result, 0, "`setvbuf()` returned `{}`", result)
}
