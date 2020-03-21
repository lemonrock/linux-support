/// Redirect standard out and standard error to syslog.
#[inline(always)]
pub fn redirect_standard_out_and_standard_error_to_syslog()
{
	redirect_to_syslog(unsafe { &mut stdout }, write_standard_out_to_syslog);
	redirect_to_syslog(unsafe { &mut stderr }, write_standard_error_to_syslog);
}
