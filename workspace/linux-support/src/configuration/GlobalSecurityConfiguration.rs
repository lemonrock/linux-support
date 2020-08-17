// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global security configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSecurityConfiguration
{
	/// If `true`, then the following in `/proc/sys/kernel` are hardened if present:-
	///
	/// * `core_pattern` (forced to `core`).
	/// * `core_pipe_limit` (set to 1 to prevent malicious user space programs exploiting a weakness in core dump captures).
	/// * `core_uses_pid` (legacy value is unset).
	/// * `dmesg_restrict`.
	/// * `ftrace_dump_on_oops`.
	/// * `kptr_restrict`.
	/// * `latencytop`.
	/// * `msg_next_id` (forced to `-1` so we aren't affected by a parent process leaving this oddly set).
	/// * `perf_event_max_stack` (forced to default of `127`).
	/// * `perf_event_paranoid`.
	/// * `print-fatal-signals`.
	/// * `printk`.
	/// * `printk_devkmsg`.
	/// * `randomize_va_space`.
	/// * `sched_child_runs_first`.
	/// * `sem_next_id` (forced to `-1` so we aren't affected by a parent process leaving this oddly set).
	/// * `shm_next_id` (forced to `-1` so we aren't affected by a parent process leaving this oddly set).
	/// * `shm_rmid_forced` (forced to `1` so unattached segments are immediately garbage-collected; will break some System V IPC programs. Tough).
	/// * `stack_erasing`.
	/// * `sysrq`.
	/// * `timer_migration`.
	/// * `traceoff_on_warning`.
	/// * `tracepoint_printk`.
	///
	/// And the following in `/proc/sys/kernel/random` are hardened if present:-
	///
	/// * `read_wakeup_threshold` (forced to default of 64 bits of entropy).
	/// * `urandom_min_reseed_secs` (forced to 15 seconds from a default of 60 seconds; ignored after Linux 4.8 but support may be readded).
	/// * `write_wakeup_threshold` (forced to default of 896 bits of entropy).
	///
	/// And the following in `/proc/sys/kernel/seccomp` are hardened if present:-
	///
	/// * `actions_logged` (forced to the value of `actions_avail` without `allow`).
	///
	/// And the following in `/proc/sys/fs` are hardened if present:-
	///
	/// * `protected_fifos`.
	/// * `protected_hardlinks`.
	/// * `protected_regular`.
	/// * `protected_symlinks`.
	/// * `suid_dumpable`.
	///
	/// And the following in `/proc/sys/net/core` are hardened if present:-
	///
	/// * `bpf_jit_kallsyms`.
	/// * `message_burst`
	/// * `message_cost`.
	/// * `tstamp_allow_data`.
	/// * (Note that `bpf_jit_harden` is treated separately below).
	///
	/// And the following in `/proc/sys/vm` are hardened if present:-
	///
	/// * `block_dump`.
	/// * `hugetlb_shm_group`.
	/// * `laptop_mode`.
	/// * `legacy_va_layout`.
	/// * `mmap_min_addr`.
	/// * `mmap_rnd_bits`.
	/// * `mmap_rnd_compat_bits`.
	/// * `vfs_cache_pressure`.
	/// * `oom_dump_tasks`.
	/// * `unprivileged_userfaultfd`.
	/// * `vfs_cache_pressure`.
	///
	/// And the following in `/proc/sys/abi` are hardened if present:-
	///
	/// * `vsyscall32` (turned off).
	///
	/// And the following in `/proc/sys/debug` are hardened if present:-
	///
	/// * `exception-trace` (turned on).
	/// * `kprobes-optimization` (turned on).
	///
	/// And the following in `/proc/sys/dev/scsi` are hardened if present:-
	///
	/// * `logging_level` (disabled).
	///
	/// And the maximum number of process identifiers is set to 2^22 in `/proc/sys/kernel/pid_max` to reduce the impact of races and process identifier wrap-around (Frankly, they should just be an UUID and be done with it but we're stuck with design decisions from 40 years ago).
	pub harden: bool,
	
	/// This hardening control prevents the creation of namespaces by setting the following files to `0` in `/proc/sys/user`:-
	///
	/// * `max_cgroup_namespaces`.
	/// * `max_ipc_namespaces`.
	/// * `max_mnt_namespaces`.
	/// * `max_net_namespaces`.
	/// * `max_pid_namespaces`.
	/// * `max_user_namespaces`.
	/// * `max_uts_namespaces`.
	///
	/// If not using containers then this value should be `true`.
	pub disable_namespaces: bool,
	
	/// Default value is 100,000.
	///
	/// A value like 20 is more sensible unless the system is being used for containers.
	pub maximum_file_system_mounts: Option<u32>,
	
	/// Default value is 65,536.
	///
	/// A value like 1,000 is probably more sensible.
	pub maximum_memory_maps_per_proces: Option<u32>,
	
	/// Hardens JIT'd eBPF programs using the file `/proc/sys/net/core/bpf_jit_harden`.
	pub harden_jit_ebpf: Option<JustInTimeCompilationHardening>,

	/// Disables kexec loading of new kernel images until reboot.
	///
	/// By default it is enabled.
	pub disable_kexec_loading_of_new_kernel_images_until_reboot: bool,

	/// Disables loading of BPF programs by unprivileged users (those lacking the capability `CAP_SYS_ADMIN`) until reboot.
	///
	/// By default it is enabled.
	pub disable_bpf_loading_of_programs_by_unprivileged_users_until_reboot: bool,

	/// Lock down state to apply until reboot.
	///
	/// By default the lock down state is `Off` (`none`).
	pub lock_down_state: LockDownState,
}

impl GlobalSecurityConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalSecurityConfigurationError>
	{
		use self::GlobalSecurityConfigurationError::*;

		#[inline(always)]
		fn harden_value_u8<'a>(proc_path: &ProcPath, file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, file_name: &'static str, value: u8) -> Result<(), GlobalSecurityConfigurationError>
		{
			harden_value(proc_path, file_function, file_name, UnpaddedDecimalInteger(value))
		}
		
		#[inline(always)]
		fn harden_value_i8<'a>(proc_path: &ProcPath, file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, file_name: &'static str, value: i8) -> Result<(), GlobalSecurityConfigurationError>
		{
			harden_value(proc_path, file_function, file_name, UnpaddedDecimalInteger(value))
		}
		
		#[inline(always)]
		fn harden_value_u32<'a>(proc_path: &ProcPath, file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, file_name: &'static str, value: u32) -> Result<(), GlobalSecurityConfigurationError>
		{
			harden_value(proc_path, file_function, file_name, UnpaddedDecimalInteger(value))
		}
		
		#[inline(always)]
		fn hardden_seccomp_logging(proc_path: &ProcPath) -> Result<(), GlobalSecurityConfigurationError>
		{
			let actions_available = proc_path.sys_kernel_seccomp_file_path("actions_avail").read_raw_without_line_feed().map_err(|cause| CouldNotHarden { cause, proc_sys_kernel_file: "actions_avail" })?;
			
			let mut actions_to_log = Vec::with_capacity(actions_available.len());
			for action in actions_available.split_bytes(b' ')
			{
				if action == b"allow"
				{
					continue
				}
				if likely!(!actions_to_log.is_empty())
				{
					actions_to_log.push(b' ');
				}
				actions_to_log.extend_from_slice(action)
			}
			actions_to_log.push(b'\n');
			
			proc_path.sys_kernel_seccomp_file_path("actions_logged").write_value(actions_to_log).map_err(|cause| CouldNotHarden { cause, proc_sys_kernel_file: "actions_logged" })
		}
		
		#[inline(always)]
		fn harden_value<'a>(proc_path: &ProcPath, file_function: impl FnOnce(&ProcPath, &str) -> PathBuf, file_name: &'static str, value: impl IntoLineFeedTerminatedByteString<'a>) -> Result<(), GlobalSecurityConfigurationError>
		{
			let file_path = file_function(proc_path, file_name);
			if file_path.exists()
			{
				return file_path.write_value(value).map_err(|cause| CouldNotHarden { cause, proc_sys_kernel_file: file_name })
			}
			Ok(())
		}
		
		#[inline(always)]
		fn set_sys_kernel_boolean_value_once(proc_path: &ProcPath, file_name: &str, value: bool, error: impl FnOnce(io::Error) -> GlobalSecurityConfigurationError) -> Result<(), GlobalSecurityConfigurationError>
		{
			set_value_once
			(
				proc_path,
				|proc_path|
				{
					let file_path = proc_path.sys_kernel_file_path(file_name);
					if file_path.exists()
					{
						let enabled: bool = file_path.read_zero_or_one_bool().unwrap();
						if !enabled
						{
							return file_path.write_value(true)
						}
					}
					Ok(())
				},
				value,
				error
			)
		}
		
		if self.harden
		{
			const RateLimit: u8 = 5;
			const RateLimitBurst: u8 = 10;
			
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "core", b"core\n" as &[u8])?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "core_pipe_limit", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "core_uses_pid", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "dmesg_restrict", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "ftrace_dump_on_oops", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "kptr_restrict", 2)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "latencytop", 0)?;
			harden_value_i8(proc_path, ProcPath::sys_kernel_file_path, "msg_next_id", -1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "perf_event_max_stack", 127)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "perf_event_paranoid", 2)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "print-fatal-signals", 0)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "printk", b"0 4 0 0\n" as &[u8])?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "printk_delay", 0)?;
			harden_value(proc_path, ProcPath::sys_kernel_file_path, "printk_devkmsg", b"off\n" as &[u8])?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "printk_ratelimit", RateLimit)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "printk_ratelimit_burst", RateLimitBurst)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "randomize_va_space", 2)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "sched_child_runs_first", 0)?;
			harden_value_i8(proc_path, ProcPath::sys_kernel_file_path, "sem_next_id", -1)?;
			harden_value_i8(proc_path, ProcPath::sys_kernel_file_path, "shm_next_id", -1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "shm_rmid_forced", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "stack_erasing", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "sysrq", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "timer_migration", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "traceoff_on_warning", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_file_path, "tracepoint_printk", 0)?;
			
			const ReadWakeUpThresholdInBitsOfEntry: u32 = 64;
			const WriteWakeUpThresholdInBitsOfEntry: u32 = 896;
			harden_value_u32(proc_path, ProcPath::sys_kernel_random_file_path, "read_wakeup_threshold", ReadWakeUpThresholdInBitsOfEntry)?;
			harden_value_u8(proc_path, ProcPath::sys_kernel_random_file_path, "urandom_min_reseed_secs", 15)?;
			harden_value_u32(proc_path, ProcPath::sys_kernel_random_file_path, "write_wakeup_threshold", WriteWakeUpThresholdInBitsOfEntry)?;
			
			hardden_seccomp_logging(proc_path)?;
			
			harden_value_u8(proc_path, ProcPath::sys_fs_file_path, "protected_fifos", 2)?;
			harden_value_u8(proc_path, ProcPath::sys_fs_file_path, "protected_hardlinks", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_fs_file_path, "protected_regular", 2)?;
			harden_value_u8(proc_path, ProcPath::sys_fs_file_path, "protected_symlinks", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_fs_file_path, "suid_dumpable", 0)?;
			
			harden_value_u8(proc_path, ProcPath::sys_net_core_file_path, "bpf_jit_kallsyms", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_net_core_file_path, "message_burst", RateLimitBurst)?;
			harden_value_u8(proc_path, ProcPath::sys_net_core_file_path, "message_cost", RateLimit)?;
			harden_value_u8(proc_path, ProcPath::sys_net_core_file_path, "tstamp_allow_data", 0)?;
			
			#[cfg(target_arch = "aarch64")] const CONFIG_ARCH_MMAP_RND_BITS_MAX: u8 = 19;
			#[cfg(target_arch = "riscv64")] const CONFIG_ARCH_MMAP_RND_BITS_MAX: u8 = 24;
			#[cfg(target_arch = "x86_64")] const CONFIG_ARCH_MMAP_RND_BITS_MAX: u8 = 32;
			#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))] const CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MAX: u8 = 16;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "block_dump", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "hugetlb_shm_group", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "laptop_mode", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "legacy_va_layout", 0)?;
			harden_value_u32(proc_path, ProcPath::sys_vm_file_path, "mmap_min_addr", 65536)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "mmap_rnd_bits", CONFIG_ARCH_MMAP_RND_BITS_MAX)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "mmap_rnd_compat_bits", CONFIG_ARCH_MMAP_RND_COMPAT_BITS_MAX)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "oom_dump_tasks", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "unprivileged_userfaultfd", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_vm_file_path, "vfs_cache_pressure", 100)?;
			
			harden_value_u8(proc_path, ProcPath::sys_abi_file_path, "vsyscall32", 0)?;
			
			harden_value_u8(proc_path, ProcPath::sys_debug_file_path, "exception-trace", 1)?;
			harden_value_u8(proc_path, ProcPath::sys_debug_file_path, "kprobes-optimization", 1)?;
			
			harden_value_u8(proc_path, ProcPath::sys_dev_scsi_file_path, "logging_level", 0)?;
			
			ProcessIdentifier::set_maximum_value_to_maximum(proc_path).map_err(CouldNotSetMaximumProcessIdentifiersToMaximum)?;
		}
		
		if self.disable_namespaces
		{
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_cgroup_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_ipc_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_mnt_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_net_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_pid_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_user_namespaces", 0)?;
			harden_value_u8(proc_path, ProcPath::sys_user_file_path, "max_uts_namespaces", 0)?;
		}
		
		set_proc_sys_fs_value(proc_path, "mount-max", self.maximum_file_system_mounts.map(UnpaddedDecimalInteger), CouldNotSetMaximumNumberOfFileSystemMounts)?;
		
		set_proc_sys_vm_value(proc_path, "max_map_count", self.maximum_memory_maps_per_proces.map(UnpaddedDecimalInteger), CouldNotSetMaximumNumberOfMemoryMapsPerProcess)?;
		
		set_value(proc_path, |proc_path, value| value.set_value(proc_path), self.harden_jit_ebpf, CouldNotHardenJitOfBpfPrograms)?;
		
		set_sys_kernel_boolean_value_once(proc_path, "kexec_load_disabled", self.disable_kexec_loading_of_new_kernel_images_until_reboot, CouldNotDisableKexecLoadingUntilNextReboot)?;
		
		set_sys_kernel_boolean_value_once(proc_path, "unprivileged_bpf_disabled", self.disable_bpf_loading_of_programs_by_unprivileged_users_until_reboot, CouldNotDisableKexecLoadingUntilNextReboot)?;
		
		self.lock_down_state.set(sys_path).map_err(CouldNotChangeLockDownState)?;
		
		Ok(())
	}
}
