// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global file descriptor configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalFileDescriptorConfiguration
{
	/// Default is 1,048,576.
	///
	/// Requires root.
	pub maximum_number_of_open_file_descriptors: Option<u64>,
}

impl GlobalFileDescriptorConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalFileDescriptorConfigurationError>
	{
		use self::GlobalFileDescriptorConfigurationError::*;

		set_value(proc_path, ResourceLimit::set_maximum_number_of_open_file_descriptors, self.maximum_number_of_open_file_descriptors, CouldNotChangeMaximumNumberOfOpenFileDescriptors)?;

		Ok(())
	}
}
