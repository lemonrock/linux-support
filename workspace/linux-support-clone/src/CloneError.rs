// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


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
