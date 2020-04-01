// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// POSIX message queue constraints.
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosixMessageQueueConstraints
{
	/// Maximum number of enqueued messages.
	pub maximum_number_of_enqueued_messages: usize,

	/// Maximum message size in bytes.
	pub maximum_message_size_in_bytes: usize,
}
