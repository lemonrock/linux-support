/// Converts a hash set of signals to a libc `sigset_t`.
#[inline(always)]
pub fn hash_set_to_signal_set(signals: &HashSet<i32>) -> sigset_t
{
	unsafe
	{
		#[allow(deprecated)] let mut signal_set: sigset_t = uninitialized();
		sigemptyset(&mut signal_set);
		for signal in signals.iter()
			{
				sigaddset(&mut signal_set, *signal);
			}
		signal_set
	}
}
