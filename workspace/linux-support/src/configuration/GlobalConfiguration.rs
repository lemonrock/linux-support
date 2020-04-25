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

		Ok(())
	}
}

// Global configuration:-
/*

MQs
	Modify message queue code to allow the following to be changed:-
		/proc/sys/fs/mqueue (since Linux 2.6.6)
	  This directory contains files msg_max, msgsize_max, and
	  queues_max, controlling the resources used by POSIX message
	  queues.  See mq_overview(7) for details.


Inotify
	/proc/sys/fs/inotify (since Linux 2.6.13)
	  This directory contains files max_queued_events,
	  max_user_instances, and max_user_watches, that can be used to
	  limit the amount of kernel memory consumed by the inotify
	  interface.  For further details, see inotify(7).

Resource Limits
	/proc/sys/fs/nr_open
	/proc/sys/fs/file-max

EPoll
	/proc/sys/fs/epoll/max_user_watches

Security
 /proc/sys/kernel/panic
 /proc/sys/kernel/panic_on_oops
 /proc/sys/kernel/pid_max
 /proc/sys/kernel/randomize_va_space
 /proc/sys/kernel/sysrq
 /proc/sys/vm/unprivileged_userfaultfd
 /proc/sys/kernel/modules_disabled
  /proc/sys/kernel/kptr_restrict
  /proc/sys/kernel/dmesg_restrict
   /proc/sys/fs/protected_symlinks
   /proc/sys/fs/protected_hardlinks
   /proc/sys/fs/mount-max
   /proc/sys/fs/dir-notify-enable

Memory
	/proc/sys/vm/admin_reserve_kbytes
	/proc/sys/vm/compact_memory
	/proc/sys/vm/drop_caches
	/proc/sys/vm/user_reserve_kbytes
	/proc/sys/vm/swappiness

SysV chared memory
	/proc/sys/kernel/msgmax
	/proc/sys/kernel/msgmni
	/proc/sys/kernel/msgmnb

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
