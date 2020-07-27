// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receives replies.
pub trait ReplyReceiver<Protocol: NetlinkProtocol>
{
	/// Starts of a message or messages (if multipart).
	fn start_of_set_of_messages(&mut self, message_identification: &MultipartMessagePartIdentification);
	
	/// Could not start to retrieve messages (usually due to an error with `recv()`).
	fn could_not_start_messages(&mut self, error: io::Error);
	
	/// Could not continue to retrieve messages (usually due to an error with `recv()`).
	fn could_not_continue_multipart_messages(&mut self, error: io::Error);
	
	/// Could not continue to retrieve messages (usually due to `recv()` returning `0`).
	fn unexpected_end_of_set_of_multipart_messages(&mut self);
	
	/// A message.
	///
	/// May be called, once, never or many times after `start_of_set_of_messages()`.
	fn message(&mut self, message_type: Protocol::MessageType, data: &[u8]);
	
	/// End of a set of messages, called after either `start_of_set_of_messages()` or `message()`.
	///
	/// `Ok(true)` if dump was interrrupted.
	/// `Ok(false)` if everything was good or an acknowledgment was received.
	/// `Err(Errno)` for an error code from the netlink request message processing code.
	fn end_of_set_of_messages(&mut self, result: Result<bool, Errno>);
}
