/// Caught an unwind.
///
/// Log it to to syslog.
#[inline(always)]
pub fn caught_unwind_and_log_it_to_syslog(panic_payload: &(dyn Any + 'static + Send))
{
	let hyper_thread = to_c_string_robustly(format!("{}", HyperThread::current_hyper_thread().0));

	let cause = to_c_string_robustly(panic_payload_to_cause(panic_payload));

	unsafe { syslog(LOG_ERR, b"HyperThread:%s:Cause:%s\0".as_ptr() as *const _, hyper_thread.as_ptr(), cause.as_ptr()) }
}
