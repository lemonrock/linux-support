// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process configuration.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessConfiguration
{
	/// Inclusive minimum.
	pub minimum_linux_kernel_version: LinuxKernelVersionNumber,

	/// CPU feature checks to suppress.
	#[serde(default)] pub cpu_feature_checks_to_suppress: CpuFeatureChecksToSuppress,

	/// Process name.
	#[serde(default)] pub name: ProcessName,

	/// Process scheduling.
	#[serde(default)] pub process_scheduling_configuration: ProcessSchedulingConfiguration,
}

impl ProcessConfiguration
{
	/// Configure.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), ProcessConfigurationError>
	{
		use self::ProcessConfigurationError::*;

		if LinuxKernelVersion::parse(proc_path).map_err(|cause| CouldNotParseLinuxKernelVersion(cause)) < self.minimum_linux_kernel_version
		{
			return Err(LinuxKernelVersionIsTooOld)
		}

		let cpu_features = CpuFeatures::validate_minimal_cpu_features(&self.cpu_feature_checks_to_suppress).map_err(|string| CpuFeatureChecksFailed(string))?;

		self.name.set_process_name(ProcessIdentifierChoice::Current, proc_path).map_err(|cause| CouldNotSetProcessName(cause))?;

		self.process_scheduling_configuration.configure(proc_path)?;

		disable_dumpable();

		lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities();

		// Current thread can use this instead of gettid()
		// TODO: /proc/thread-self

		// TODO: SecComp
		// TODO: Minimum capabilities to launch with.
		// TODO: Linux modules to load
		// TODO: umask
		// TODO: personality verification
		// TODO: Resource limits
		// TODO: Mounts (hugetlbfs, cpusets; really need to mount them in /dev)

		Ok(())
	}
}
