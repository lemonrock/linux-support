// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A seccomp eBPF program.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeccompProgram(BpfProgram);

impl Deref for SeccompProgram
{
	type Target = BpfProgram;

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
	/// The calling thread must either have the `CAP_SYS_ADMIN` capability or the call to `no_new_privileges()` (inside this logic) must succeed.
	///
	/// The Linux kernel must have been compiled with `CONFIG_SECCOMP_FILTER`.
	///
	/// Returns `Ok(Err(ThreadIdentifier))` if using `SeccompFilterFlags::SynchronizeAllThreads` and another thread could not be synchronized.
	/// Returns `Ok(Ok(())` if everything succeeded.
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn load(mut self, flags: SeccompFilterFlags) -> io::Result<Result<(), ThreadIdentifier>>
	{
		let length = self.len();
		debug_assert!(length <= BPF_MAXINSNS as usize, "seccomp programs are limited to a maximum of BPF_MAXINSNS ({}) instructions; this program is {} instructions", BPF_MAXINSNS, length);
		let mut program = sock_fprog
		{
			len: length as u16,
			filter: self.as_mut_ptr(),
		};

		no_new_privileges()?;

		// If `flags.bits == 0` we prefer the older `prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &program)` system call as it is more likely to be supported.
		let result = if flags.bits == 0
		{
			unsafe { prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &program) }
		}
		else
		{
			seccomp(SECCOMP_SET_MODE_FILTER, flags.bits, &mut program as *mut sock_fprog as *mut _)
		};
		if likely!(result >= 0)
		{
			if result > 0
			{
				if flags.contains(SeccompFilterFlags::SynchronizeAllThreads)
				{
					Ok(Err(ThreadIdentifier::from(unsafe { NonZeroI32::new_unchecked(result) })))
				}
				else
				{
					unreachable!("Unexpected result {} from seccomp()", result)
				}
			}
			else
			{
				Ok(Ok(()))
			}
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			if flags == SeccompFilterFlags::empty()
			{
				unreachable!("Unexpected result {} from prctl()", result)
			}
			else
			{
				unreachable!("Unexpected result {} from seccomp()", result)
			}
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
	pub fn disallow_only_these_syscalls(&mut self, syscalls: &IndexSet<SYS>, undefined: &Vec<Range<u32>>)
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
	pub fn disallow_only_these_syscalls_256_or_fewer(&mut self, syscalls: &IndexSet<SYS>, undefined: &Vec<Range<u32>>)
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
		self.jump_if_greater_than_or_equal_to_constant(SYS::InclusiveMinimum as usize as u32, 1, 0);
		self.return_kill_the_process()
	}

	#[inline(always)]
	fn disallow_upper(&mut self)
	{
		self.jump_if_greater_than_constant(SYS::InclusiveMaximum as usize as u32, 0, 1);
		self.return_kill_the_process()
	}

	/// Not efficient.
	#[inline(always)]
	pub fn allow_only_these_syscalls(&mut self, syscalls: &IndexSet<SYS>)
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
	pub fn allow_only_these_syscalls_256_or_fewer(&mut self, syscalls: &IndexSet<SYS>)
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
