// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The only system calls that the calling thread is permitted to make are `read()`, `write()`, `_exit()` (but not `exit_group()`), and `sigreturn()`.
///
/// Other system calls result in the delivery of a `SIGKILL` signal.
/// Strict secure computing mode is useful for number-crunching applications that may need to execute untrusted byte code, perhaps obtained by reading from a pipe or socket.
///
/// Note that although the calling thread can no longer call `sigprocmask()`, it can use `sigreturn()` to block all signals apart from `SIGKILL` and `SIGSTOP`.
/// This means that `alarm()` (for example) is not sufficient for restricting the process's execution time.
/// Instead, to reliably terminate the process, `SIGKILL` must be used.
/// This can be done by using `timer_create()` with `SIGEV_SIGNAL` and `sigev_signo` set to `SIGKILL`, or by using `setrlimit()` to set the hard limit for `RLIMIT_CPU`.
#[inline(always)]
pub fn strict_seccomp() -> io::Result<()>
{
	let result = seccomp(SECCOMP_SET_MODE_STRICT, 0, null_mut());
	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		Err(io::Error::last_os_error())
	}
	else
	{
		unreachable_code(format_args!("seccomp() returned unexpected result {}", result))
	}
}
