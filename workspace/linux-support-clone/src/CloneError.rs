/// Clone error
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CloneError
{
	/// Too many processes are already running.
	MaximumNumberOfProcessesExceeded,

	/// Cannot allocate sufficient memory to allocate a task structure for the child, or to copy those parts of the caller's context that need to be copied.
	InsufficientMemoryToAllocateChildTask,

	/// One of:-
	///
	/// * `CLONE_NEWPID` was specified in flags, but the limit on the nesting depth of process identifier (PID) namespaces would have been exceeded; see `man 7 pid_namespaces`. Since Linux 3.7.
	/// * `CLONE_NEWUSER` was specified in flags, and the call would cause the limit on the number of nested user namespaces to be exceeded; see `man 7 user_namespaces`. Since Linux 4.9.
	/// * One of the values in flags specified the creation of a new user namespace, but doing so would have caused the limit defined by the corresponding file in /proc/sys/user to be exceeded. For further details see `man 7 namespaces`. Since Linux 4.9.
	NestingOfUserOrProcessIdentifierNamespacesWouldBeExceeded,
}

impl Display for CloneError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for CloneError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::CloneError::*;

		match self
		{
			&MaximumNumberOfProcessesExceeded => None,

			&InsufficientMemoryToAllocateChildTask => None,

			&NestingOfUserOrProcessIdentifierNamespacesWouldBeExceeded => None,
		}
	}
}
