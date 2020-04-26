// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalConfiguration
{
	/// Requires root.
	pub global_scheduling: GlobalSchedulingConfiguration,

	/// Requires root.
	pub pipe: GlobalPipeConfiguration,

	/// Requires root.
	pub file_leasing: GlobalFileLeasingConfiguration,

	/// Requires root.
	pub posix_message_queue: GlobalPosixMessageQueueConfiguration,

	/// Requires root.
	pub system_v_message_queue: GlobalSystemVMessageQueueConfiguration,

	/// Requires root.
	pub inotify: GlobalInotifyConfiguration,

	/// Requires root.
	pub epoll: GlobalEPollConfiguration,

	/// Requires root.
	pub linux_kernel_asynchronous_io: GlobalLinuxKernelAsynchronousIoConfiguration,

	/// Requires root.
	pub file_handle: GlobalFileHandleConfiguration,

	/// Requires root.
	pub file_descriptor: GlobalFileDescriptorConfiguration,

	/// Requires root.
	pub linux_module: GlobalLinuxModuleConfiguration,

	/// Requires root.
	pub kernel_panic: GlobalKernelPanicConfiguration,
}

impl GlobalConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalConfigurationError>
	{
		self.global_scheduling.configure(sys_path, proc_path)?;

		self.pipe.configure(proc_path)?;

		self.file_leasing.configure(proc_path)?;

		self.posix_message_queue.configure(proc_path)?;

		self.system_v_message_queue.configure(proc_path)?;

		self.inotify.configure(proc_path)?;

		self.epoll.configure(proc_path)?;

		self.linux_kernel_asynchronous_io.configure(proc_path)?;

		self.file_handle.configure(proc_path)?;

		self.file_descriptor.configure(proc_path)?;

		self.linux_module.configure(proc_path)?;

		self.kernel_panic.configure(proc_path)?;

		Ok(())
	}
}


// Global configuration:-
/*


		// TODO: LinuxKernelCommandLineValidator (a mess currently)

Mounts
	/dev/mqueue
	/dev/cpuset
	/dev/hugetlbfs
Security: Mounts
   /proc/sys/fs/mount-max

NUMA

numa_balancing

Enables/disables automatic page fault based NUMA memory balancing. Memory is moved automatically to nodes that access it often.

Enables/disables automatic NUMA memory balancing. On NUMA machines, there is a performance penalty if remote memory is accessed by a CPU. When this feature is enabled the kernel samples what task thread is accessing memory by periodically unmapping pages and later trapping a page fault. At the time of the page fault, it is determined if the data being accessed should be migrated to a local memory node.

The unmapping of pages and trapping faults incur additional overhead that ideally is offset by improved memory locality but there is no universal guarantee. If the target workload is already bound to NUMA nodes then this feature should be disabled. Otherwise, if the system overhead from the feature is too high then the rate the kernel samples for NUMA hinting faults may be controlled by the numa_balancing_scan_period_min_ms, numa_balancing_scan_delay_ms, numa_balancing_scan_period_max_ms, numa_balancing_scan_size_mb, and numa_balancing_settle_count sysctls.
numa_balancing_scan_period_min_ms, numa_balancing_scan_delay_ms, numa_balancing_scan_period_max_ms, numa_balancing_scan_size_mb

Security
 /proc/sys/kernel/randomize_va_space
 /proc/sys/kernel/sysrq
 /proc/sys/kernel/stack_erasing (if present)
  /proc/sys/kernel/kptr_restrict
  /proc/sys/kernel/dmesg_restrict
   /proc/sys/fs/suid_dumpable
   /proc/sys/fs/protected_symlinks
   /proc/sys/fs/protected_hardlinks
protected_fifos
protected_hardlinks
protected_regular
protected_symlinks

(only if present)
/proc/sys/fs/kexec_load_disabled


System V shared memory

shmall
shmmax
shmmni
shm_rmid_forced
msg_next_id, sem_next_id, and shm_next_id (System V IPC)

(only if present)
hung_task_check_count
hung_task_timeout_secs
hung_task_check_interval_secs
hung_task_warnings




Security: Process identifiers
 /proc/sys/kernel/pid_max
Security: userfaultfd
 /proc/sys/vm/unprivileged_userfaultfd

Memory
	/proc/sys/vm/admin_reserve_kbytes
	/proc/sys/vm/user_reserve_kbytes
	/proc/sys/vm/compact_memory
	/proc/sys/vm/drop_caches
	/proc/sys/vm/swappiness

Memory / Machine Checks
	/proc/sys/vm/memory_failure_early_kill
	/proc/sys/vm/memory_failure_recovery

OOM
	/proc/sys/vm/oom_dump_tasks
	/proc/sys/vm/oom_kill_allocating_task
	/proc/sys/vm/overcommit_kbytes
	/proc/sys/vm/overcommit_memory
	/proc/sys/vm/overcommit_ratio
	/proc/sys/vm/panic_on_oom

*/
