// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

		Ok(())
	}
}

// Global configuration:-
/*

Resource Limits
	/proc/sys/fs/nr_open


Security
 /proc/sys/kernel/panic
 /proc/sys/kernel/panic_on_oops
 /proc/sys/kernel/randomize_va_space
 /proc/sys/kernel/sysrq
  /proc/sys/kernel/kptr_restrict
  /proc/sys/kernel/dmesg_restrict
   /proc/sys/fs/protected_symlinks
   /proc/sys/fs/protected_hardlinks
protected_fifos
protected_hardlinks
protected_regular
protected_symlinks


   /proc/sys/fs/suid_dumpable
Security: Process identifiers
 /proc/sys/kernel/pid_max
Security: userfaultfs
 /proc/sys/vm/unprivileged_userfaultfd
Security: Modules
 /proc/sys/kernel/modules_disabled
Security: Mounts
   /proc/sys/fs/mount-max

Memory
	/proc/sys/vm/admin_reserve_kbytes
	/proc/sys/vm/compact_memory
	/proc/sys/vm/drop_caches
	/proc/sys/vm/user_reserve_kbytes
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
