// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global file leasing configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalFileLeasingConfiguration
{
	/// Are leases enabled or disabled?
	///
	/// Requires root.
	pub leases_enabled: Option<bool>,

	/// Grace time in seconds.
	///
	/// Requires root.
	pub number_of_seconds_a_lease_holder_has_to_release_a_lease: Option<usize>
}

impl GlobalFileLeasingConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalFileLeasingConfigurationError>
	{
		use self::GlobalFileLeasingConfigurationError::*;

		if let Some(leases_enabled) = self.leases_enabled
		{
			change_leases_enabled(proc_path, leases_enabled).map_err(|cause| CouldNotChangeLeasesEnabled(cause))
		}

		if let Some(number_of_seconds_a_lease_holder_has_to_release_a_lease) = self.number_of_seconds_a_lease_holder_has_to_release_a_lease
		{
			set_number_of_seconds_a_lease_holder_has_to_release_a_lease(proc_path, number_of_seconds_a_lease_holder_has_to_release_a_lease).map_err(|cause| CouldNotChangeNumberOfSecondsALeaseHolderHasToReleaseALease(cause))
		}

		Ok(())
	}
}
