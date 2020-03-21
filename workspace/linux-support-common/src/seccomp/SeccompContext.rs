/// A Seccomp context.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeccompContext(NonNull<scmp_filter_ctx>);

impl Drop for SeccompContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { seccomp_release(self.context()) }
	}
}

impl SeccompContext
{
	/// New instance.
	#[inline(always)]
	pub fn new(default_action: Action) -> Result<Self, ()>
	{
		let context = unsafe { seccomp_init(default_action.to_u32()) };
		if unlikely!(context.is_null())
		{
			Err(())
		}
		else
		{
			Ok(Self(unsafe { NonNull::new_unchecked(context) }))
		}
	}

	/// Set action to take on bad architecture.
	#[inline(always)]
	pub fn set_bad_architecture_action(&self, action: Action) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_ACT_BADARCH, action.to_u32())
	}

	/// Set `NO_NEW_PRIVS` on filter load.
	#[inline(always)]
	pub fn set_no_new_privileges_on_filter_load(&self, enable: bool) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_CTL_NNP, enable as u32)
	}

	/// Synchronize all threads.
	#[inline(always)]
	pub fn synchronize_all_threads_with_same_seccomp_context_on_filter_load(&self, enable: bool) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_CTL_TSYNC, enable as u32)
	}

	/// Allow tracing.
	#[inline(always)]
	pub fn allow_tracing_with_syscall_number_negative_one(&self, enable: bool) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_API_TSKIP, enable as u32)
	}

	/// Log not allowed actions.
	#[inline(always)]
	pub fn log_not_allowed_actions(&self, enable: bool) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_CTL_LOG, enable as u32)
	}

	/// Disable speculative store bypass.
	#[inline(always)]
	pub fn disable_speculative_store_bypass(&self, enable: bool) -> Result<(), ()>
	{
		self.set_attribute(scmp_filter_attr::SCMP_FLTATR_CTL_SSB, enable as u32)
	}

	#[inline(always)]
	fn set_attribute(&self, attribute: scmp_filter_attr, value: u32) -> Result<(), ()>
	{
		let result = unsafe { seccomp_attr_set(self.context(), attribute, value) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(())
		}
		else
		{
			panic!("Positive result from `seccomp_load()` of `{}`", result)
		}
	}

	/// Set Architecture.
	#[inline(always)]
	pub fn set_architecture(&self, architecture: scmp_arch) -> Result<(), ()>
	{
		let result = unsafe { seccomp_arch_add(self.context(), architecture as u32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(())
		}
		else
		{
			panic!("Positive result from `seccomp_arch_add()` of `{}`", result)
		}
	}

	/// `system_call_number` should be a value of `__NR_syscall` for the current architecture.
	#[inline(always)]
	pub fn set_system_call_priority(&self, system_call_number: i32, priority: u8) -> Result<(), ()>
	{
		let result = unsafe { seccomp_syscall_priority(self.context(), system_call_number, priority) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(())
		}
		else
		{
			panic!("Positive result from `seccomp_syscall_priority()` of `{}`", result)
		}
	}

	/// `system_call_number` should be a value of `__NR_syscall` for the current architecture.
	#[inline(always)]
	pub fn add_rule_for_system_call(&self, system_call_number: i32, action: Action, comparisons: &[Comparison]) -> Result<(), ()>
	{
		let comparators: Vec<_> = comparisons.iter().map(|comparison| comparison.to_scmp_arg_cmp()).collect();

		let result = unsafe { seccomp_rule_add_exact_array(self.context(), action.to_u32(), system_call_number, comparators.len().try_into().unwrap(), comparators.as_ptr()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(())
		}
		else
		{
			panic!("Positive result from `seccomp_rule_add_exact_array()` of `{}`", result)
		}
	}

	/// Load.
	#[inline(always)]
	pub fn load(self) -> Result<(), ()>
	{
		let result = unsafe { seccomp_load(self.context()) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result < 0)
		{
			Err(())
		}
		else
		{
			panic!("Positive result from `seccomp_load()` of `{}`", result)
		}
	}

	#[inline(always)]
	fn context(&self) -> *mut scmp_filter_ctx
	{
		self.0.as_ptr()
	}
}
