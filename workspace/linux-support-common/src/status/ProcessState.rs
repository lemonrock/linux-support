/// Process state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessState
{
	/// Also known as `R (running)`.
	Running,

	/// Also known as `S (sleeping)`.
	Sleeping,

	/// Also known as `D (disk sleep)`, or disk sleep.
	SleepingInAnUninterruptibleWait,

	/// Also known as `T (stopped)`.
	TracedOrStopped,

	/// Also known as `t (tracing stop)`.
	TracingStop,

	/// Also known as `X (dead)`.
	Dead,

	/// Also known as `Z (zombie)`.
	Zombie,

	/// Also known as `P (parked)`.
	Parked,

	/// Also known as `I (idle)`.
	Idle,
}