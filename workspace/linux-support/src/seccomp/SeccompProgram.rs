// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A seccomp BPF program.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeccompProgram(ClassicBpfProgram);

impl Deref for SeccompProgram
{
	type Target = ClassicBpfProgram;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for SeccompProgram
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl SeccompProgram
{
	/// # `log`
	///
	/// All filter return actions except `SECCOMP_RET_ALLOW` should be logged in the kernel audit log (available if the kernel booted with `audit=1`).
	///
	/// An administrator may override this filter flag by preventing specific actions from being logged via the `/proc/sys/kernel/seccomp/actions_logged` file.
	///
	/// Since Linux 4.14.
	///
	///
	/// # `disable_speculative_store_bypass_mitigation`
	///
	/// Disable Speculative Store Bypass mitigation.
	///
	/// Since Linux 4.17.
	#[inline(always)]
	pub fn load(self, log: bool, disable_speculative_store_bypass_mitigation: bool) -> io::Result<()>
	{
		self.load_internal(log, disable_speculative_store_bypass_mitigation, true, false, (), |non_zero_positive_i32| unexpected_result!(seccomp, non_zero_positive_i32.get() as usize))
	}

	/// The calling thread must either have the `CAP_SYS_ADMIN` capability or the call to `no_new_privileges()` (inside this logic) must succeed.
	///
	/// The Linux kernel must have been compiled with `CONFIG_SECCOMP_FILTER`.
	///
	///
	/// # `log`
	///
	/// All filter return actions except `SECCOMP_RET_ALLOW` should be logged in the kernel audit log (available if the kernel booted with `audit=1`).
	///
	/// An administrator may override this filter flag by preventing specific actions from being logged via the `/proc/sys/kernel/seccomp/actions_logged` file.
	///
	/// Since Linux 4.14.
	///
	///
	/// # `disable_speculative_store_bypass_mitigation`
	///
	/// Disable Speculative Store Bypass mitigation.
	///
	/// Since Linux 4.17.
	///
	///
	/// When adding a new filter, all other threads of the calling process are synchronized to the same seccomp filter tree.
	///
	/// A "filter tree" is the ordered list of filters attached to a thread.
	/// (Attaching identical filters in separate `seccomp()` calls results in different filters from this perspective).
	///
	/// If any thread cannot synchronize to the same filter tree, the call will not attach the new seccomp filter, and will fail, returning the first thread ID found that cannot synchronize.
	/// Synchronization will fail if another thread in the same process is in `SECCOMP_MODE_STRICT` or if it has attached new seccomp filters to itself, diverging from the calling thread's filter tree.
	///
	///
	/// Returns `Ok(Err(ThreadIdentifier))` and another thread could not be synchronized.
	/// Returns `Ok(Ok(())` if everything succeeded.
	#[inline(always)]
	pub fn load_and_synchronize_all_threads(self, log: bool, disable_speculative_store_bypass_mitigation: bool) -> io::Result<Result<(), ThreadIdentifier>>
	{
		self.load_internal(log, disable_speculative_store_bypass_mitigation, true, false, Ok(()), |non_zero_positive_i32| Err(ThreadIdentifier::from(non_zero_positive_i32)))
	}

	/// # `log`
	///
	/// All filter return actions except `SECCOMP_RET_ALLOW` should be logged in the kernel audit log (available if the kernel booted with `audit=1`).
	///
	/// An administrator may override this filter flag by preventing specific actions from being logged via the `/proc/sys/kernel/seccomp/actions_logged` file.
	///
	/// Since Linux 4.14.
	///
	///
	/// # `disable_speculative_store_bypass_mitigation`
	///
	/// Disable Speculative Store Bypass mitigation.
	///
	/// Since Linux 4.17.
	#[inline(always)]
	pub fn load_and_accept_user_notification(self, log: bool, disable_speculative_store_bypass_mitigation: bool) -> io::Result<SeccompUserNotificationFileDescriptor>
	{
		self.load_internal(log, disable_speculative_store_bypass_mitigation, true, false, SeccompUserNotificationFileDescriptor(0), |non_zero_positive_i32| SeccompUserNotificationFileDescriptor(non_zero_positive_i32.get()))
	}

	#[inline(always)]
	fn load_internal<O>(mut self, log: bool, disable_speculative_store_bypass_mitigation: bool, synchronize_all_threads: bool, new_listener: bool, zero: impl FnOnce() -> O, non_zero_positive_i32: impl FnOnce(NonZeroI32) -> O) -> io::Result<O>
	{
		debug_assert!(synchronize_all_threads != new_listener && new_listener != true);

		let flags = if log
		{
			SECCOMP_FILTER_FLAG_LOG
		}
		else
		{
			0
		}
		| if disable_speculative_store_bypass_mitigation
		{
			SECCOMP_FILTER_FLAG_SPEC_ALLOW
		}
		else
		{
			0
		}
		| if synchronize_all_threads
		{
			SECCOMP_FILTER_FLAG_TSYNC
		}
		else
		{
			0
		}
		| if new_listener
		{
			SECCOMP_FILTER_FLAG_NEW_LISTENER
		}
		else
		{
			0
		};

		let length = self.len();
		debug_assert!(length <= BPF_MAXINSNS as usize, "seccomp programs are limited to a maximum of BPF_MAXINSNS ({}) instructions; this program is {} instructions", BPF_MAXINSNS, length);
		let mut program = sock_fprog
		{
			len: length as u16,
			filter: self.as_mut_ptr(),
		};

		match system_call_seccomp(SECCOMP_SET_MODE_FILTER, flags, &mut program as *mut sock_fprog as *mut _).as_usize()
		{
			0 => Ok(zero()),
			
			ok @ 1 ..= SystemCallResult::I32Maximum_usize => non_zero_positive_i32(new_non_zero_i32(ok as i32)),
			
			error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => Err(SystemCallResult::usize_to_system_call_error_number(error).into()),
			
			unexpected @ _ => unexpected_result!(seccomp, unexpected),
		}
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn validate_syscall_architecture(&mut self, architecture: AuditArchitecture)
	{
		self.load_syscall_architecture();
		self.jump_if_equal_to_constant(architecture as u32, 1, 0);
		self.return_kill_the_process()
	}

	/// Not efficient.
	#[inline(always)]
	pub fn disallow_only_these_syscalls(&mut self, syscalls: &IndexSet<SystemCallNumber>, undefined: &Vec<Range<u32>>)
	{
		self.load_syscall_number();

		self.disallow_lower();
		self.disallow_upper();
		self.disallow_undefined(undefined);

		for sys in syscalls.iter()
		{
			let system_call_number = (*sys) as usize as u32;
			self.jump_if_equal_to_constant(system_call_number, 0, 1);
			self.return_kill_the_process()
		}

		self.return_allow()
	}

	/// Higher code density than `disallow_only_these_syscalls()`.
	#[inline(always)]
	pub fn disallow_only_these_syscalls_256_or_fewer(&mut self, syscalls: &IndexSet<SystemCallNumber>, undefined: &Vec<Range<u32>>)
	{
		self.load_syscall_number();

		self.disallow_lower();
		self.disallow_upper();
		self.disallow_undefined(undefined);

		let mut jump_statements_until_get_to_return_kill_the_process = syscalls.len();
		debug_assert!(jump_statements_until_get_to_return_kill_the_process <= 256, "This approach only supports 256 or fewer syscalls at a time");
		for sys in syscalls.iter()
		{
			jump_statements_until_get_to_return_kill_the_process -= 1;
			let jump_over_return_kill_the_process_if_last_statement = if unlikely!(jump_statements_until_get_to_return_kill_the_process == 0)
			{
				1
			}
			else
			{
				0
			};
			let system_call_number = (*sys) as usize as u32;
			self.jump_if_equal_to_constant(system_call_number, jump_statements_until_get_to_return_kill_the_process as u8, jump_over_return_kill_the_process_if_last_statement);
		}

		self.return_kill_the_process();
		self.return_allow()
	}

	#[inline(always)]
	fn disallow_undefined(&mut self, undefined: &Vec<Range<u32>>)
	{
		for range in undefined
		{
			self.disallow_undefined_range(range)
		}
	}

	#[inline(always)]
	fn disallow_undefined_range(&mut self, range: &Range<u32>)
	{
		let inclusive_undefined: u32 = range.start;
		let exclusive_undefined: u32 = range.end;
		debug_assert!(inclusive_undefined < exclusive_undefined);

//		if system_call >= inclusive_undefined
//		{
//			if system_call >= exclusive_undefined
//			{
//				continue
//			}
//			else
//			{
//				return self.return_kill_the_process()
//			}
//		}
//		else
//		{
//			continue
//		}

		const ContinueAfterStatements: u8 = 2;
		self.jump_if_greater_than_or_equal_to_constant(inclusive_undefined, 0, ContinueAfterStatements);
		self.jump_if_greater_than_or_equal_to_constant(exclusive_undefined, ContinueAfterStatements - 1, 0);
		self.return_kill_the_process()
	}

	#[inline(always)]
	fn disallow_lower(&mut self)
	{
		self.jump_if_greater_than_or_equal_to_constant(SystemCallNumber::InclusiveMinimum as usize as u32, 1, 0);
		self.return_kill_the_process()
	}

	#[inline(always)]
	fn disallow_upper(&mut self)
	{
		self.jump_if_greater_than_constant(SystemCallNumber::InclusiveMaximum as usize as u32, 0, 1);
		self.return_kill_the_process()
	}

	/// Not efficient.
	#[inline(always)]
	pub fn allow_only_these_syscalls(&mut self, syscalls: &IndexSet<SystemCallNumber>)
	{
		self.load_syscall_number();

		for sys in syscalls.iter()
		{
			let system_call_number = (*sys) as usize as u32;
			self.jump_if_equal_to_constant(system_call_number, 0, 1);
			self.return_allow()
		}

		self.return_kill_the_process()
	}

	/// Higher code density than `allow_only_these_syscalls()`.
	#[inline(always)]
	pub fn allow_only_these_syscalls_256_or_fewer(&mut self, syscalls: &IndexSet<SystemCallNumber>)
	{
		self.load_syscall_number();

		let mut jump_statements_until_get_to_allow = syscalls.len();
		debug_assert!(jump_statements_until_get_to_allow <= 256, "This approach only supports 256 or fewer syscalls at a time");
		for sys in syscalls.iter()
		{
			jump_statements_until_get_to_allow -= 1;
			let jump_over_allow_if_last_statement = if unlikely!(jump_statements_until_get_to_allow == 0)
			{
				1
			}
			else
			{
				0
			};
			let system_call_number = (*sys) as usize as u32;
			self.jump_if_equal_to_constant(system_call_number, jump_statements_until_get_to_allow as u8, jump_over_allow_if_last_statement);
		}

		self.return_allow();
		self.return_kill_the_process()
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn load_syscall_number(&mut self)
	{
		const nr: usize = offset_of!(seccomp_data, nr);
		self.load_accumulator_with_fixed_offset_struct_field(nr)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn load_syscall_architecture(&mut self)
	{
		const architecture: usize = offset_of!(seccomp_data, arch);
		self.load_accumulator_with_fixed_offset_struct_field(architecture)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn return_kill_the_process(&mut self)
	{
		self.return_constant(SyscallOutcome::KillTheProcess.bits, 0)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn return_kill_the_thread(&mut self)
	{
		self.return_constant(SyscallOutcome::KillTheThread.bits, 0)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn return_allow(&mut self)
	{
		self.return_constant(SyscallOutcome::Allow.bits, 0)
	}
}
