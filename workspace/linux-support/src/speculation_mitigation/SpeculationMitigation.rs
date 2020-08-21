// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Per-Thread Speculation Control.
	#[derive(Serialize, Deserialize)]
	pub struct SpeculationMitigation: i32
	{
		const PerThreadMitigationControlPossible = PR_SPEC_PRCTL;
		
		/// The speculation feature is enabled, mitigation is disabled.
		const SpeculationEnabled = PR_SPEC_ENABLE;
		
		/// The speculation feature is disabled, mitigation is enabled.
		const SpeculationDisabled = PR_SPEC_ENABLE;
		
		/// The speculation feature is disabled, mitigation is enabled; this can not be undone.
		const ForceSpeculationDisabled = PR_SPEC_FORCE_DISABLE;
		
		/// The speculation feature is disabled, mitigation is enabled but the state will be cleared on `execve()`.
		const NoExecSpeculationDisabled = PR_SPEC_DISABLE_NOEXEC;
	}
}

impl SpeculationMitigation
{
	/// Store bypass speculation mitigation for the current thread.
	#[inline(always)]
	pub fn store_bypass() -> Result<Self, Errno>
	{
		Self::current(PR_SPEC_STORE_BYPASS)
	}
	
	/// Indirect branch speculation mitigation for the current thread.
	#[inline(always)]
	pub fn indirect_branch() -> Result<Self, Errno>
	{
		Self::current(PR_SPEC_INDIRECT_BRANCH)
	}
	
	#[inline(always)]
	fn current(subcommand: usize) -> Result<Self, Errno>
	{
		process_control_wrapper2(PR_GET_SPECULATION_CTRL,subcommand,|non_negative_result| Ok(Self::from_bits_truncate(non_negative_result)),Err)
	}
}
