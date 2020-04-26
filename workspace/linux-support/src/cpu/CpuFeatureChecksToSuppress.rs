// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU feature checks which can be suppressed.
///
/// Current names (and typical warning messages) are:-
///
/// * `spectre_v2_google`: "Kernel has `spectre_v2=retpoline,google`; this is probably not the best choice".
/// * `has_rep_movsb_stosb`: "Your CPU does not support the REP MOVSB and REP STOSB instructions, which are optimal for some memory moves and copies".
/// * `has_prefetchw`: "Your CPU does not support the PRETFCHW instruction, which is optimal for some memory moves and copies".
/// * `has_ss`: "Your CPU does not support self-snoop of the cache (which nearly all should), which is important for efficient cache management in this application".
/// * `has_working_xsave`: "CPU architecture either lacks XSAVE support or the Linux kernel has disabled XSAVE support".
/// * `has_invpcid`: "Your CPU does not support the INVPCID instruction, which is important for efficient mitigation of the Meltdown and Spectre security vulnerabilities".
/// * `has_smap`: "Your CPU does not support the Supervisor-Mode Access Prevention (SMAP) instructions CLAC and STAC, which are important for securing modern Linux systems".
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CpuFeatureChecksToSuppress(HashSet<&'static str>);

impl CpuFeatureChecksToSuppress
{
	// Development on Mac Pro `trash cans` at this time assumes at least Intel Ivy Bridge CPUs.
	#[inline(always)]
	pub(crate) fn performance_warnings_it_is_safe_to_assume_for_all_x86_64_cpu_architectures_as_of_q2_2018(&self, failed_checks: &mut FailedChecks, feature_information: &FeatureInfo, extended_function_information: &ExtendedFunctionInfo, extended_feature_information: &ExtendedFeatures)
	{
		failed_checks.check(&self.0, "has_rep_movsb_stosb", "Your CPU does not support the REP MOVSB and REP STOSB instructions, which are optimal for some memory moves and copies", || extended_feature_information.has_rep_movsb_stosb());

		failed_checks.check(&self.0, "has_prefetchw", "Your CPU does not support the PRETFCHW instruction, which is optimal for some memory moves and copies", || extended_function_information.has_prefetchw());

		failed_checks.check(&self.0, "has_ss", "Your CPU does not support self-snoop of the cache (which nearly all should), which is important for efficient cache management in this application", || feature_information.has_ss());

		failed_checks.check(&self.0, "has_working_xsave", "CPU architecture either lacks XSAVE support or the Linux kernel has disabled XSAVE support", || feature_information.has_xsave() && feature_information.has_oxsave())
	}

	#[inline(always)]
	pub(crate) fn performance_warnings_for_new_features(&self, failed_checks: &mut FailedChecks, feature_information: &FeatureInfo, extended_feature_information: &ExtendedFeatures)
	{
		failed_checks.check(&self.0, "has_invpcid", "Your CPU does not support the INVPCID instruction, which is important for efficient mitigation of the Meltdown and Spectre security vulnerabilities", || feature_information.has_xsave() && extended_feature_information.has_invpcid())
	}

	#[inline(always)]
	pub(crate) fn security_warnings_for_new_features(&self, failed_checks: &mut FailedChecks, extended_feature_information: &ExtendedFeatures)
	{
		failed_checks.check(&self.0, "has_smap", "Your CPU does not support the Supervisor-Mode Access Prevention (SMAP) instructions CLAC and STAC, which are important for securing modern Linux systems", || extended_feature_information.has_smap())
	}
}
