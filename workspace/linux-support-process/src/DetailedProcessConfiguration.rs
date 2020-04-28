// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Common process configuration.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DetailedProcessConfiguration
{
	/// Whitelist of capabilities to retain.
	///
	/// eg:-
	/// * `SystemAdministration`.
	/// * `LockMemory`.
	/// * `BindPortsBelow1024`.
	/// * `SetUid`.
	/// * `SetGid`.
	/// * `Nice`.
	#[serde(default)] pub capabilities_to_retain: HashSet<Capability>,
}

impl Default for DetailedProcessConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			capabilities_to_retain: HashSet::default(),
		}
	}
}

impl DetailedProcessConfiguration
{
	#[inline(always)]
	fn configure(&self, valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>, daemonize: Option<&Daemonize>) -> Result<(), DetailedProcessConfigurationError>
	{
		Self::set_current_process_affinity(&valid_hyper_threads_for_the_current_process)?;
	}

	#[inline(always)]
	fn set_current_process_affinity(valid_hyper_threads_for_the_current_process: &BTreeSet<HyperThread>) -> Result<(), DetailedProcessConfigurationError>
	{
		let cpu_set = CpuSet::from(valid_hyper_threads_for_the_current_process);
		cpu_set.set_current_process_affinity().map_err(DetailedProcessConfigurationError::CouldNotSetCurrentProcessAffinity)
	}
}
