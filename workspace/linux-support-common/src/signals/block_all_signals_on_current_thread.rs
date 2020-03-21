/// Block all signals on the current thread.
#[inline(always)]
pub fn block_all_signals_on_current_thread()
{
	block_all_signals_on_current_thread_bar(&HashSet::default())
}
