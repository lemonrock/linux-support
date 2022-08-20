// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Privilege protection.
///
/// Prevents a thread, when executing, `clone()`, `fork()` and particularly `execve()` from gaining additional capabilities from, say setuid/setgid programs or programs with file capabilities.
///
/// This MUST be called prior to `seccomp(SECCOMP_SET_MODE_FILTER)` if the current thread does not the `CAP_SYS_ADMIN` capability.
#[inline(always)]
pub fn change_no_new_privileges(enable_or_disable_no_new_privileges: bool) -> Result<(), SystemCallErrorNumber>
{
	let value = if enable_or_disable_no_new_privileges
	{
		1
	}
	else
	{
		0
	};
	process_control_wrapper2(PR_SET_NO_NEW_PRIVS,value,result_must_be_zero, Err)
}
