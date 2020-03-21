/// Log a fatal signal number to syslog before exiting.
#[inline(always)]
pub fn log_exit_signalled_to_syslog(signal_number: Option<SignalNumber>)
{
	match signal_number
	{
		None => unsafe { syslog(LOG_NOTICE, b"ExitSignalled:Other\0".as_ptr() as *const _) },

		Some(signal_number) => unsafe { syslog(LOG_NOTICE, b"ExitSignalled:%s\0".as_ptr() as *const _, strsignal(signal_number)) },
	}
}
