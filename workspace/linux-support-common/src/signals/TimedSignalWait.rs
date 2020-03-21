/// Represents the result of waiting for a set of signals.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TimedSignalWait
{
	/// Timed out.
	TimedOut,

	/// Signalled.
	Signalled(SignalNumber),

	/// Other signal interrupted.
	OtherSignalInterrupted,
}
