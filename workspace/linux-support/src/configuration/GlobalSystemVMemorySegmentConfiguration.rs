// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global System V message queue configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSystemVMemorySegmentConfiguration
{
	/// Default is 18446744073692774399.
	///
	/// Requires root.
	pub maximum_memory_segment_size: Option<NonZeroU64>,

	/// Default is 4,096.
	///
	/// Requires root.
	pub maximum_number_of_memory_segment_identifiers: Option<NonZeroU32>,

	/// Default is 18446744073692774399.
	///
	/// Requires root.
	pub maximum_number_of_shared_memory_pages: Option<NonZeroNumberOfPages>,
}

impl GlobalSystemVMemorySegmentConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalSystemVMemorySegmentConfigurationError>
	{
		use self::GlobalSystemVMemorySegmentConfigurationError::*;

		set_value(proc_path, set_maximum_memory_segment_size, self.maximum_memory_segment_size, CouldNotChangeMaximumMemorySegmentSize)?;
		set_value(proc_path, set_maximum_number_of_memory_segment_identifiers, self.maximum_number_of_memory_segment_identifiers, CouldNotChangeMaximumNumberOfMemorySegmentIdentifiers)?;
		set_value(proc_path, set_maximum_number_of_shared_memory_pages, self.maximum_number_of_shared_memory_pages, CouldNotChangeMaximumNumberOfSharedMemoryPages)
	}
}
