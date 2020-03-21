/// Block all signals specified the current thread.
#[inline(always)]
pub fn block_all_signals_on_current_thread_bar(signals: &HashSet<SignalNumber>)
{
	let result = unsafe
	{
		#[allow(deprecated)] let mut set = uninitialized();
		sigfillset(&mut set);
		for signal in signals.iter()
		{
			sigdelset(&mut set, *signal);
		}
		pthread_sigmask(SIG_SETMASK, &set, null_mut())
	};

	match result
	{
		0 => (),
		-1 => panic!("pthread_sigmask returned an error"),
		_ => panic!("pthread_sigmask returned an invalid result '{}'", result)
	}
}
