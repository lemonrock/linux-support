// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Syscall outcome.
	///
	/// Unsupported actions are equivalent to `KillTheProcess`.
	pub struct SyscallOutcome: u32
	{
		#[allow(missing_docs)]
		const KillTheProcess = SECCOMP_RET_KILL_PROCESS;

		#[allow(missing_docs)]
		const KillTheThread = SECCOMP_RET_KILL_THREAD;

		/// Also known as a trap.
		const DisallowAndRaiseSigSys = SECCOMP_RET_TRAP;

		/// Return an error.
		///
		/// Bottom bits specify error number.
		const ReturnErrorNumber = SECCOMP_RET_ERRNO;

		/// Notify userspace on a file descriptor returned from `seccomp(SECCOMP_SET_MODE_FILTER, SECCOMP_FILTER_FLAG_NEW_LISTENER)`.
		const NotifyUserspace = SECCOMP_RET_USER_NOTIF;

		#[allow(missing_docs)]
		const DisallowOrPassToPTrace = SECCOMP_RET_TRACE;

		#[allow(missing_docs)]
		const AllowButLog = SECCOMP_RET_LOG;

		#[allow(missing_docs)]
		const Allow = SECCOMP_RET_ALLOW;
	}
}

impl SyscallOutcome
{
	/// Is the syscall outcome (return action) supported by the current Linux kernel?
	///
	/// Unsupported actions are equivalent to `KillTheProcess`.
	#[inline(always)]
	pub fn is_supported_by_current_linux_kernel(self) -> io::Result<bool>
	{
		let mut bits = self.bits;
		
		match system_call_seccomp(SECCOMP_GET_ACTION_AVAIL, 0, &mut bits as *mut u32 as *mut _).as_usize()
		{
			0 => Ok(true),
			
			SystemCallResult::EOPNOTSUPP_usize => Ok(false),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Err(SystemCallResult::usize_to_system_call_error_number(error).into()),
			
			unexpected @ _ => unexpected_result!(seccomp, unexpected),
		}
	}
}
