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
	/// A value such as `20` has been recommended.
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

		set_value(proc_path, change_leases_enabled, self.leases_enabled, CouldNotChangeLeasesEnabled)?;
		set_value(proc_path, set_number_of_seconds_a_lease_holder_has_to_release_a_lease, self.number_of_seconds_a_lease_holder_has_to_release_a_lease, CouldNotChangeNumberOfSecondsALeaseHolderHasToReleaseALease)
	}
}
