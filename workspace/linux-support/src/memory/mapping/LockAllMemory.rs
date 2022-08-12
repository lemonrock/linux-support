// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Lock all memory in the process' virtual address space from being swapped.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum LockAllMemory
{
	/// All current memory.
	AllCurrentMemory = MCL_CURRENT,

	/// All current memory on fault.
	AllCurrentMemoryOnFault = MCL_CURRENT | MCL_ONFAULT,

	/// All future memory.
	AllFutureMemory = MCL_FUTURE,

	/// All future memory on fault.
	AllFutureMemoryOnFault = MCL_FUTURE | MCL_ONFAULT,

	/// All current and future memory.
	AllCurrentAndFutureMemory = MCL_CURRENT | MCL_FUTURE,

	/// All current and future memory on fault.
	AllCurrentAndFutureMemoryOnFault = MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT,
}

impl LockAllMemory
{
	/// Lock.
	///
	/// Should be called after rlimits have been changed as can have quite a deleterious effect.
	#[inline(always)]
	pub fn lock(self)
	{
		let flags = self as i32;
		let result = unsafe { mlockall(flags) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => panic!("the caller had a nonzero RLIMIT_MEMLOCK soft resource limit, but tried to lock more memory than the limit permitted. This limit is not enforced if the process is privileged (CAP_IPC_LOCK)."),
				EPERM => panic!("The caller is not privileged, but needs privilege (CAP_IPC_LOCK) to perform the requested operation."),
				EINVAL => panic!("Unknown flags were specified or MCL_ONFAULT was specified without either MCL_FUTURE or MCL_CURRENT"),

				unexpected @ _ => panic!("Unexpected error {} from mlockall()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from mlockall()", result))
		}
	}

	/// Unlock.
	pub fn unlock()
	{
		let result = unsafe { munlockall() };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno()
			{
				ENOMEM => panic!("the caller had a nonzero RLIMIT_MEMLOCK soft resource limit, but tried to lock more memory than the limit permitted. This limit is not enforced if the process is privileged (CAP_IPC_LOCK)."),
				EPERM => panic!("The caller is not privileged, but needs privilege (CAP_IPC_LOCK) to perform the requested operation. Or, (Linux 2.6.8 and earlier) The caller was not privileged (CAP_IPC_LOCK)"),
				EINVAL => panic!("Unknown flags were specified or MCL_ONFAULT was specified without either MCL_FUTURE or MCL_CURRENT"),

				unexpected @ _ => panic!("Unexpected error {} from munlockall()", unexpected)
			}
		}
		else
		{
			unreachable_code(format_args!("Unexpected result {} from munlockall()", result))
		}
	}
}
