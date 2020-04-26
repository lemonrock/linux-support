// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global inotify configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalInotifyConfiguration
{
	/// Default is 16,384.
	///
	/// Requires root.
	pub maximum_number_of_events_that_can_be_queued: Option<NonZeroU32>,

	/// Default is 128.
	///
	/// Requires root.
	pub maximum_number_of_inotify_instances_per_user: Option<NonZeroU32>,

	/// Default is 8,192.
	///
	/// Requires root.
	pub maximum_number_of_watches_per_user: Option<NonZeroU32>,
}

impl GlobalInotifyConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalInotifyConfigurationError>
	{
		use self::GlobalInotifyConfigurationError::*;

		set_value(proc_path, set_maximum_number_of_events_that_can_be_queued, self.maximum_number_of_events_that_can_be_queued, CouldNotChangeMaximumNumberOfEventsThatCanBeQueued)?;
		set_value(proc_path, set_maximum_number_of_inotify_instances_per_user, self.maximum_number_of_inotify_instances_per_user, CouldNotChangeMaximumNumberOfInotifyInstancesPerUser)?;
		set_value(proc_path, set_maximum_number_of_watches_per_user, self.maximum_number_of_watches_per_user, CouldNotChangeMaximumNumberOfWatchesPerUser)
	}
}
