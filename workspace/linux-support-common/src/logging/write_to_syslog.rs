#[inline(always)]
fn write_to_syslog(priority: c_int, data: *const c_char, length: size_t) -> ssize_t
{
	const SyslogFormat: ConstCStr = ConstCStr(b"%s:%s\0");

	unsafe { syslog(priority, SyslogFormat.as_ptr(), length, program_invocation_short_name, data) };
	length as ssize_t
}
