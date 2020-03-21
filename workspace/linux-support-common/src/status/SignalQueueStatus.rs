/// Signal queue status.
#[derive(Default, Debug, Copy,Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalQueueStatus
{
	/// Number of signals queued.
	pub number_of_signals_queued: u64,

	/// Maximum number of signals that can be queued (maximum queue depth).
	pub maximum_number_of_signals_that_can_be_queued: u64,
}