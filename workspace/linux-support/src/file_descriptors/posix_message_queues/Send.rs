// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Receive a message to a queue.
pub trait Send: PosixMessageQueue
{
	/// Sends a message.
	///
	/// A message may be empty.
	fn send(&self, message_buffer: &[u8], message_priority: PosixMessagePriority) -> Result<(), StructWriteError>;
}
