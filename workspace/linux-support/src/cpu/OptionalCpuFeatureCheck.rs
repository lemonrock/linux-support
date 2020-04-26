// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An optional CPU feature check.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumMessage, EnumIter, IntoStaticStr)]
pub enum OptionalCpuFeatureCheck
{
	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018", detailed_message = "Your CPU does not support the REP MOVSB and REP STOSB instructions, which are optimal for some memory moves and copies")]
	has_rep_movsb_stosb,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018", detailed_message = "Your CPU does not support the PRETFCHW instruction, which is optimal for some memory moves and copies")]
	has_prefetchw,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018", detailed_message = "Your CPU does not support self-snoop of the cache (which nearly all should), which is important for efficient cache management in this application")]
	has_ss,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018", detailed_message = "CPU architecture either lacks XSAVE support or the Linux kernel has disabled XSAVE support")]
	has_working_xsave,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018", detailed_message = "CPU architecture does not support Time Stamp Counter (TSC) adjust Model Specific Registers (MSR)")]
	has_tsc_adjust_msr,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "performance_warnings_for_new_features", detailed_message = "Your CPU does not support the INVPCID instruction, which is important for efficient mitigation of the Meltdown and Spectre security vulnerabilities")]
	has_invpcid,

	#[cfg(target_arch = "x86_64")]
	#[allow(missing_docs)]
	#[strum(message = "security_warnings_for_new_features", detailed_message = "Your CPU does not support the Supervisor-Mode Access Prevention (SMAP) instructions CLAC and STAC, which are important for securing modern Linux systems")]
	has_smap,
}

#[cfg(target_arch = "x86_64")]
impl Check for OptionalCpuFeatureCheck
{
	const Name: &'static str = "Optional CPU feature check";

	type CheckArguments = (FeatureInfo, ExtendedFunctionInfo, ExtendedFeatures);

	#[inline(always)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool
	{
		let &(ref feature_information, ref extended_function_information, ref extended_feature_information) = check_arguments;

		use self::OptionalCpuFeatureCheck::*;

		match self
		{
			has_rep_movsb_stosb => extended_feature_information.has_rep_movsb_stosb(),
			has_prefetchw => extended_function_information.has_prefetchw(),
			has_ss => feature_information.has_ss(),
			has_working_xsave => feature_information.has_xsave() && feature_information.has_oxsave(),
			has_tsc_adjust_msr => extended_feature_information.has_tsc_adjust_msr(),
			has_invpcid => feature_information.has_xsave() && extended_feature_information.has_invpcid(),
			has_smap => extended_feature_information.has_smap(),
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
