// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global EPoll configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalFileHandleConfiguration
{
	/// Default varies; might be 99,642 on a system with 1Gb.
	///
	/// Requires root.
	pub maximum_number_of_file_handles: Option<NumberOfFileHandles>,
}

impl GlobalFileHandleConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalFileHandleConfigurationError>
	{
		use self::GlobalFileHandleConfigurationError::*;

		set_value(proc_path, |proc_path, value| value.set_maximum(proc_path), self.maximum_number_of_file_handles, CouldNotChangeMaximumNumberOfFileHandles)
	}
}
