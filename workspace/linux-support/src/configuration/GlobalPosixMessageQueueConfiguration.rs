// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global POSIX message queue configuration.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalPosixMessageQueueConfiguration
{
	/// Can not exceed 65,536 (`HARD_MSGMAX`).
	///
	/// Default is 10.
	///
	/// Requires root.
	pub default_maximum_number_of_messages_in_a_queue: Option<NonZeroU32>,

	/// Can not exceed 65,536 (`HARD_MSGMAX`).
	///
	/// Default is 10.
	///
	/// Requires root.
	pub maximum_maximum_number_of_messages_in_a_queue: Option<NonZeroU32>,

	/// Can not exceed 16,777,216 (`HARD_MSGSIZEMAX`).
	///
	/// Default is 8,192.
	/// Minimum is 128.
	///
	/// Requires root.
	pub default_maximum_message_size: Option<NonZeroU32>,

	/// Can not exceed 16,777,216 (`HARD_MSGSIZEMAX`).
	///
	/// Default is 8,192.
	/// Minimum is 128.
	///
	/// Requires root.
	pub maximum_maximum_message_size: Option<NonZeroU32>,

	/// Prior to Linux 3.14 could not exceed 1,024 (`HARD_QUEUESMAX`).
	///
	/// Default is 256.
	///
	/// Requires root.
	pub maximum_number_of_queues: Option<NonZeroU32>,
}

impl GlobalPosixMessageQueueConfiguration
{
	/// Configures.
	pub fn configure(&self, proc_path: &ProcPath) -> Result<(), GlobalPosixMessageQueueConfigurationError>
	{
		use self::GlobalPosixMessageQueueConfigurationError::*;

		set_value(proc_path, set_default_maximum_number_of_messages_in_a_queue, self.default_maximum_number_of_messages_in_a_queue, CouldNotChangeDefaultMaximumNumberOfMessagesInAQueue)?;
		set_value(proc_path, set_maximum_maximum_number_of_messages_in_a_queue, self.maximum_maximum_number_of_messages_in_a_queue, CouldNotChangeMaximumMaximumNumberOfMessagesInAQueue)?;
		set_value(proc_path, set_default_maximum_message_size, self.default_maximum_message_size, CouldNotChangeDefaultMaximumMessageSize)?;
		set_value(proc_path, set_maximum_maximum_message_size, self.maximum_maximum_message_size, CouldNotChangeMaximumMaximumMessageSize)?;
		set_value(proc_path, set_maximum_number_of_queues, self.maximum_number_of_queues, CouldNotChangeMaximumNumberOfQueues)
	}
}
