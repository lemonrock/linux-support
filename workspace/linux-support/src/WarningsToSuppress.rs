// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU and Kernel missing feature warnings to suppress.
#[derive(Default, Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct WarningsToSuppress
{
	/// Missing Kernel features whose warnings should be supressed.
	///
	/// Current names (and typical warning messages) are:-
	///
	/// * `hashdist`: Warnings about `hashdist=0`.
	/// * `noaliencache`: "Kernel has `noaliencache` enabled; this is likely to hurt performance".
	/// * `numa_zonelist_order`: "Kernel has `noaliencache` enabled; this is likely to hurt performance".
	/// * `skew_tick`: "Kernel should have `skew_tick=1` for maximum performance at the cost of power consumption".
	/// * `idle_poll`: "Warnings about idle!=poll".
	/// * `noxsaveopt`: "Kernel has `noxsaveopt` enabled; this is likely to hurt performance".
	pub suppress_warnings_for_kernel_features: HashSet<String>,
}

impl WarningsToSuppress
{
	#[inline(always)]
	pub(crate) fn kernel_warn<F: FnOnce() -> bool>(&self, name: &str, message: &str, true_if_should_not_warn: F)
	{
		if true_if_should_not_warn()
		{
			return
		}

		if self.suppress_warnings_for_kernel_features.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}

	#[inline(always)]
	pub(crate) fn kernel_warn_without_check(&self, name: &str, message: &str)
	{
		if self.suppress_warnings_for_kernel_features.contains(name)
		{
			return
		}

		LoggingConfiguration::warn(name, format!("{}", message))
	}
}
