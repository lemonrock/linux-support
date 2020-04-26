// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global EPoll configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalEPollConfiguration
{
	/// Default varies; might be 204,328 on a system with 1Gb.
	///
	/// Requires root.
	pub maximum_number_of_watched_file_descriptors_per_user: Option<NonZeroU32>,
}

impl GlobalEPollConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalEPollConfigurationError>
	{
		use self::GlobalEPollConfigurationError::*;

		set_value(proc_path, set_maximum_number_of_watched_file_descriptors_per_user, self.maximum_number_of_watched_file_descriptors_per_user, CouldNotChangeMaximumNumberOfWatchedFileDescriptorsPerUser)
	}
}
