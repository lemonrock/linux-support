#[repr(u32)]
pub(crate) enum SCMP_ACT
{
	/// Kill a process.
	KILL_PROCESS = 0x80000000,

	/// Same as `KILL`.
	KILL_THREAD = 0x00000000,

	/// Raise a `SIGSYS` signal.
	TRAP = 0x00030000,

	/// Notify userspace.
	NOTIFY = 0x7FC00000,

	/// Allow the syscall to be executed after the action has been logged.
	LOG = 0x7FFC0000,

	/// Allow the system call.
	ALLOW = 0x7FFF0000,
}

impl SCMP_ACT
{
	#[allow(dead_code)]
	const KILL: Self = SCMP_ACT::KILL_THREAD;
}
