/// Action.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum Action
{
	/// Kill process.
	KillProcess,

	/// Kill a thread (same as `kill` command and `syscall`).
	KillThread,

	/// Raise.
	RaiseSigSysSignal,

	/// Notify.
	NotifyUserspace,

	/// Return an error.
	ReturnErrno(u16),

	/// Trace.
	Trace(u16),

	/// Log.
	Log,

	/// Allow.
	Allow,
}

impl Default for Action
{
	#[inline(always)]
	fn default() -> Self
	{
		Action::KillProcess
	}
}

impl Action
{
	/// To u32.
	#[inline(always)]
	pub fn to_u32(self) -> u32
	{
		use self::Action::*;
		use self::SCMP_ACT::*;

		match self
		{
			KillProcess => KILL_PROCESS as u32,

			KillThread => KILL_THREAD as u32,

			RaiseSigSysSignal => TRAP as u32,

			NotifyUserspace => NOTIFY as u32,

			ReturnErrno(error) => SCMP_ACT_ERRNO(error as u32),

			Trace(trace) => SCMP_ACT_TRACE(trace as u32),

			Log => LOG as u32,

			Allow => ALLOW as u32,
		}
	}
}
