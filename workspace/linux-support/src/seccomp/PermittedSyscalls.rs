// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Permitted syscalls.
///
/// Default is `PermittedSyscalls::Disallow()` and is based on Docker's default profile with the following removed as they interfere with NUMA:-
///
/// * `get_mempolicy`: Syscall that modifies kernel memory and NUMA settings. Also gated by `CAP_SYS_NICE`.
/// * `set_mempolicy`: Syscall that modifies kernel memory and NUMA settings. Also gated by `CAP_SYS_NICE`.
/// * `mbind`: Syscall that modifies kernel memory and NUMA settings. Also gated by `CAP_SYS_NICE`.
/// * `move_pages`: Syscall that modifies kernel memory and NUMA settings.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum PermittedSyscalls
{
	/// Only these syscalls will be allowed.
	Allow(IndexSet<SYS>),

	/// Disallows all `known` syscalls, all `undefined` syscalls and any syscalls below `SYS::InclusiveMinimum` and above `SYS::InclusiveMaximum`.
	Disallow
	{
		/// Known syscalls to disallow.
		known: IndexSet<SYS>,

		/// 'Undefined' syscalls to disallow.
		///
		/// See `SYS::Undefined`.
		undefined: Vec<Range<u32>>,

	},
}

impl Default for PermittedSyscalls
{
	#[inline(always)]
	fn default() -> Self
	{
		use self::SYS::*;

		PermittedSyscalls::Disallow
		{
			undefined: SYS::Undefined.to_vec(),

			known: indexset!
			{
				// Accounting syscall which could let processes disable their own resource limits or process accounting.
				// Also gated by `CAP_SYS_PACCT`.
				acct,

				// Prevent processes from using the kernel keyring, which is not namespaced.
				add_key,

				// Deny loading potentially persistent BPF programs into kernel.
				// Also gated by `CAP_SYS_ADMIN`.
				bpf,

				// Time/date is not namespaced.
				// Also gated by `CAP_SYS_TIME`.
				clock_adjtime,

				// Time/date is not namespaced.
				// Also gated by `CAP_SYS_TIME`.
				clock_settime,

				// Deny cloning new namespaces.
				// Also gated by `CAP_SYS_ADMIN` for `CLONE_*` flags, except `CLONE_USERNS`.
				clone,

				// Deny manipulation and functions on kernel modules.
				// Obsolete.
				// Also gated by `CAP_SYS_MODULE`.
				create_module,

				// Deny manipulation and functions on kernel modules.
				// Also gated by `CAP_SYS_MODULE`.
				delete_module,

				// Deny manipulation and functions on kernel modules.
				// Also gated by `CAP_SYS_MODULE`.
				finit_module,

				// Deny retrieval of exported kernel and module symbols.
				// Obsolete.
				get_kernel_syms,

				// Deny manipulation and functions on kernel modules.
				// Also gated by `CAP_SYS_MODULE`.
				init_module,

				// Prevent processes from modifying kernel I/O privilege levels.
				// Also gated by `CAP_SYS_RAWIO`.
				ioperm,

				// Prevent processes from modifying kernel I/O privilege levels.
				// Also gated by `CAP_SYS_RAWIO`.
				iopl,

				// Restrict process inspection capabilities.
				// Also gated by `CAP_PTRACE`.
				kcmp,

				// Deny loading a new kernel for later execution.
				// Also gated by `CAP_SYS_BOOT`.
				kexec_load,

				// Sister syscall of `kexec_load` that does the same thing, slightly different arguments.
				// Also gated by `CAP_SYS_BOOT`.
				kexec_file_load,

				// Prevent processes from using the kernel keyring, which is not namespaced.
				keyctl,

				// Tracing/profiling syscall, which could leak a lot of information on the host.
				// Also gated by `CAP_SYS_ADMIN`.
				lookup_dcookie,

				// Deny mounting.
				// Also gated by `CAP_SYS_ADMIN`.
				mount,

				// Sister syscall to open_by_handle_at.
				// Also gated by `CAP_DAC_READ_SEARCH`.
				name_to_handle_at,

				// Deny interaction with the kernel nfs daemon.
				// Obsolete since Linux 3.1.
				nfsservctl,

				// Cause of an old process breakout.
				// Also gated by `CAP_DAC_READ_SEARCH`.
				open_by_handle_at,

				// Tracing/profiling syscall, which could leak a lot of information on the host.
				perf_event_open,

				// Not inherently dangerous, but poorly tested, with potential for a lot of kernel vulnerabilities and no good use cases.
				personality,

				// Deny pivot_root, should be privileged operation.
				pivot_root,

				// Restrict process inspection capabilities.
				// Also gated by `CAP_PTRACE`.
				process_vm_readv,

				// Restrict process inspection capabilities.
				// Also gated by `CAP_PTRACE`.
				process_vm_writev,

				// Tracing/profiling syscall, which could leak a lot of information on the host.
				// Also gated by `CAP_PTRACE`.
				// Blocked in Linux kernel versions before 4.8 to avoid seccomp bypass.
				ptrace,

				// Deny manipulation and functions on kernel modules.
				// Obsolete.
				query_module,

				// Quota syscall which could let processes disable their own resource limits or process accounting.
				// Also gated by `CAP_SYS_ADMIN`.
				quotactl,

				// Don’t let processes reboot the host.
				// Also gated by `CAP_SYS_BOOT`.
				reboot,

				// Prevent processes from using the kernel keyring, which is not namespaced.
				request_key,

				// Deny associating a thread with a namespace.
				// Also gated by `CAP_SYS_ADMIN`.
				setns,

				// Time/date is not namespaced.
				// Also gated by `CAP_SYS_TIME`.
				settimeofday,

				// Deny start/stop swapping to file/device.
				// Also gated by `CAP_SYS_ADMIN`.
				swapon,

				// Deny start/stop swapping to file/device.
				// Also gated by `CAP_SYS_ADMIN`.
				swapoff,

				// Obsolete syscall.
				sysfs,

				// Obsolete, replaced by `/proc/sys`.
				_sysctl,

				// Should be a privileged operation.
				// Also gated by `CAP_SYS_ADMIN`.
				umount2,

				// Deny cloning new namespaces for processes.
				// Also gated by `CAP_SYS_ADMIN`, with the exception of `unshare --user`.
				unshare,

				// Older syscall related to shared libraries, unused for a long time.
				uselib,

				// Userspace page fault handling, largely needed for process migration.
				userfaultfd,

				// Obsolete syscall.
				ustat,
			}
		}
	}
}

impl PermittedSyscalls
{
	/// Convert to a Seccomp program.
	#[inline(always)]
	pub fn seccomp_program(&self) -> SeccompProgram
	{
		use self::AuditArchitecture::*;
		let architecture = if cfg!(target_arch = "aarch64")
		{
			AARCH64
		}
		else if cfg!(target_arch = "mips64")
		{
			if cfg!(target_endian = "little")
			{
				MIPSEL64
			}
			else
			{
				MIPS64
			}
		}
		else if cfg!(target_arch = "powerpc64")
		{
			if cfg!(target_endian = "little")
			{
				PPC64LE
			}
			else
			{
				PPC64
			}
		}
		else if cfg!(target_arch = "riscv64")
		{
			RISCV64
		}
		else if cfg!(target_arch = "s390x")
		{
			S390X
		}
		else if cfg!(target_arch = "sparc64")
		{
			SPARC64
		}
		else if cfg!(target_arch = "x86_64")
		{
			X86_64
		}
		else
		{
			unimplemented!("Unsupported target_arch")
		};

		let mut seccomp_program = SeccompProgram::default();
		seccomp_program.validate_syscall_architecture(architecture);

		use self::PermittedSyscalls::*;
		match self
		{
			&Allow(ref known) =>
			{
				if known.len() <= 256
				{
					seccomp_program.allow_only_these_syscalls_256_or_fewer(known)
				}
				else
				{
					seccomp_program.allow_only_these_syscalls(known)
				}
			}

			&Disallow { ref known, ref undefined } =>
			{
				if known.len() <= 256
				{
					seccomp_program.disallow_only_these_syscalls_256_or_fewer(known, undefined)
				}
				else
				{
					seccomp_program.disallow_only_these_syscalls(known, undefined)
				}
			},
		}

		seccomp_program
	}
}
