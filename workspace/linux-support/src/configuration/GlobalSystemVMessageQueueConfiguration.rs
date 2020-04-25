// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global System V message queue configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalSystemVMessageQueueConfiguration
{
	/// ?Does not exceed 16,777,216 (`HARD_MSGSIZEMAX`).
	///
	/// Default is 8,192.
	/// ?Minimum is 128.
	///
	/// Requires root.
	pub maximum_message_size: Option<NonZeroU32>,

	/// Default is 32,000.
	///
	/// Requires root.
	pub maximum_number_of_queue_identifiers: Option<NonZeroU32>,

	/// Default is 16,384.
	///
	/// Requires root.
	pub maximum_queue_size_in_bytes: Option<NonZeroU32>,
}

impl GlobalSystemVMessageQueueConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalSystemVMessageQueueConfigurationError>
	{
		use self::GlobalSystemVMessageQueueConfigurationError::*;

		if let Some(maximum_message_size) = self.default_maximum_number_of_messages_in_a_queue
		{
			set_maximum_message_size(proc_path, maximum_message_size).map_err(|cause| CouldNotChangeMaximumMessageSize(cause))
		}

		if let Some(maximum_number_of_queue_identifiers) = self.maximum_maximum_number_of_messages_in_a_queue
		{
			set_maximum_number_of_queue_identifiers(proc_path, maximum_number_of_queue_identifiers).map_err(|cause| CouldNotChangeMaximumNumberOfQueueIdentifiers(cause))
		}

		if let Some(maximum_queue_size_in_bytes) = self.default_maximum_message_size
		{
			set_maximum_queue_size_in_bytes(proc_path, maximum_queue_size_in_bytes).map_err(|cause| CouldNotChangeMaximumQueueSizeInBytes(cause))
		}

		Ok(())
	}
}
