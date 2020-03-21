#[inline(always)]
unsafe extern "C" fn write_standard_out_to_syslog(_cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t
{
	write_to_syslog(LOG_NOTICE, data, length)
}