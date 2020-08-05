// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global out-of-memory configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalOutOfMemoryConfiguration
{
	/// Defaults to false.
	///
	/// Requires root.
	pub panic_on_out_of_memory: Option<bool>,

	/// Defaults to false, but `true` is probably a better choice.
	///
	/// Requires root.
	pub kill_task_that_caused_out_of_memory: Option<bool>,

	/// Changes the over commit policy.
	///
	/// Requires root.
	pub over_commit_policy: Option<OverCommitPolicy>,
	
	/// A value of 8Mb is suitable for `OverCommitPolicy::Guess`; the sum of the resident set sizes (RSS) of essential processes (eg `sshd`, `top`, etc).
	/// A value of 128Mb is suitable for `OverCommitPolicy::Never`; the sum of the resident set sizes (RSS) of essential processes (eg `sshd`, `top`, etc) and their virtual sizes (VSZ).
	///
	/// Administrator users are those users with the `CAP_SYS_ADMIN` capability.
	///
	/// Requires root.
	pub over_commit_reserved_kilobytes_for_administrator_users: Option<Kilobytes>,
	
	/// A value of 128Mb is suitable for `OverCommitPolicy::Never`; alternatively, a value of 0Kb to allow oner user (unprivileged) process to use all memory bar that reserved by `over_commit_reserved_kilobytes_for_administrator_users`.
	///
	/// Normal users are those users without the `CAP_SYS_ADMIN` capability.
	///
	/// Requires root.
	pub over_commit_reserved_kilobytes_for_normal_users: Option<Kilobytes>,
}

impl GlobalOutOfMemoryConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalOutOfMemoryConfigurationError>
	{
		use self::GlobalOutOfMemoryConfigurationError::*;

		set_proc_sys_vm_value(proc_path, "panic_on_oom", self.panic_on_out_of_memory, CouldNotChangePanicOnOutOfMemory)?;
		set_proc_sys_vm_value(proc_path, "oom_kill_allocating_task", self.kill_task_that_caused_out_of_memory, CouldNotChangeKillTaskThatCausedOutOfMemory)?;
		set_value(proc_path, |proc_path, value| value.set(proc_path), self.over_commit_policy, CouldNotChangeMemoryOverCommitPolicy)?;
		set_proc_sys_vm_value(proc_path, "admin_reserve_kbytes", self.over_commit_reserved_kilobytes_for_administrator_users, CouldNotChangeMemoryOverCommitReservedKilobytesForAdministratorUsers)?;
		set_proc_sys_vm_value(proc_path, "user_reserve_kbytes", self.over_commit_reserved_kilobytes_for_normal_users, CouldNotChangeMemoryOverCommitReservedKilobytesForNormalUsers)?;

		Ok(())
	}
}
