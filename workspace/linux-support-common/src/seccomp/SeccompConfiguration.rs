// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Seccomp configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct SeccompConfiguration
{
	/// Default action.
	#[serde(default)] pub default_action: Action,

	/// Default bad architecture action.
	#[serde(default)] pub bad_architecture_action: Action,

	/// Log not-allowed actions.
	#[serde(default)] pub log_not_allowed_actions: bool,

	/// System call names to rules.
	#[serde(default)] pub rules: HashMap<CString, Rule>,
}

impl SeccompConfiguration
{
	/// Load.
	#[inline(always)]
	pub fn load(&self) -> Result<(), ()>
	{
		let seccomp_context = SeccompContext::new(self.default_action)?;

		seccomp_context.set_architecture(scmp_arch::SCMP_ARCH_NATIVE)?;

		seccomp_context.set_bad_architecture_action(self.bad_architecture_action)?;
		seccomp_context.set_no_new_privileges_on_filter_load(true)?;
		seccomp_context.synchronize_all_threads_with_same_seccomp_context_on_filter_load(true)?;
		seccomp_context.allow_tracing_with_syscall_number_negative_one(false)?;
		seccomp_context.log_not_allowed_actions(self.log_not_allowed_actions)?;
		seccomp_context.disable_speculative_store_bypass(false)?;

		for (ref system_call_name, &Rule { action_to_take, priority, ref comparisons }) in self.rules.iter()
		{
			let system_call_number = unsafe { seccomp_syscall_resolve_name_arch(scmp_arch::SCMP_ARCH_NATIVE as u32, system_call_name.as_ptr()) };

			if unlikely!(system_call_number < 0)
			{
				match system_call_number
				{
					-1 => panic!("`seccomp_syscall_resolve_name_arch()` has no system call number for system call name `{:?}`", system_call_name),
					__NR_SCMP_ERROR => panic!("`seccomp_syscall_resolve_name_arch()` returned an error for system call name `{:?}`", system_call_name),
					_ => panic!("`seccomp_syscall_resolve_name_arch()` returned an unexpected result `{}` for system call name `{:?}`", system_call_number, system_call_name),
				}
			}

			seccomp_context.set_system_call_priority(system_call_number, priority)?;
			seccomp_context.add_rule_for_system_call(system_call_number, action_to_take, &comparisons)?;
		}

		seccomp_context.load()
	}
}
