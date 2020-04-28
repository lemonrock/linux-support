// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An optional kernel command line setting check.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumMessage, EnumIter, IntoStaticStr)]
#[serde(deny_unknown_fields)]
pub enum OptionalKernelCommandLineSettingCheck
{
	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel should be booted with `hashdist=0` to disable NUMA hash distribution")]
	hashdist,

	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel has `noaliencache` enabled; this is likely to hurt performance")]
	noaliencache,

	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel should have `skew_tick=1` for maximum performance at the cost of power consumption")]
	skew_tick,

	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel has `numa_zonelist_order` enabled; this is likely to hurt performance")]
	numa_zonelist_order,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel has `noxsaveopt` enabled; this is likely to hurt performance")]
	noxsaveopt,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance", detailed_message = "Kernel should be booted with `idle=poll` for maximum performance at the cost of power consumption")]
	idle_poll,
}

impl Check for OptionalKernelCommandLineSettingCheck
{
	const Name: &'static str = "Optional CPU feature check";

	type CheckArguments = LinuxKernelCommandLineParameters;

	#[inline(always)]
	fn check(self, kernel_command_line_parameters: &Self::CheckArguments) -> bool
	{
		use self::OptionalKernelCommandLineSettingCheck::*;

		match self
		{
			hashdist => kernel_command_line_parameters.hashdist() == Some(false),
			noaliencache => !self.noaliencache(),
			skew_tick => kernel_command_line_parameters.skew_tick() == Some(true),
			numa_zonelist_order => kernel_command_line_parameters.numa_zonelist_order().is_none(),
			#[cfg(target_arch = "x86_64")] noxsaveopt => !kernel_command_line_parameters.noxsaveopt(),
			#[cfg(target_arch = "x86_64")] idle_poll => kernel_command_line_parameters.idle() == Some(b"poll"),
		}
	}
}

#[cfg(not(target_arch = "x86_64"))]
impl Check for OptionalCpuFeatureCheck
{
	type CheckArguments = ();

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		true
	}
}
