// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Receive a message from a queue.
pub trait Receive: PosixMessageQueue
{
	/// Returns a tuple of `(message_size, message_priority)`.
	///
	/// Fails with a panic if the `message_buffer` is too small for the queue's configured message size (use `PosixMessageQueue::queue_attributes()` to find this).
	fn receive(&self, message_buffer: &mut [u8]) -> Result<(usize, PosixMessagePriority), StructReadError>;
}
