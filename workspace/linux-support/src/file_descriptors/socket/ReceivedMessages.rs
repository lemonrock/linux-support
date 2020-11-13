// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Collection-like object for receiving many UDP messages at once.
pub struct ReceivedMessages<'a, SD: SocketData>
{
	received_message_helpers: Vec<ReceivedMessageHelper<'a, SD>>,
	multi_message_headers: Vec<mmsghdr>,
}

impl<'a, SD: SocketData> ReceivedMessages<'a, SD>
{
	/// Creates a new instance, wrapping the `receive_buffers`.
	pub fn new(receive_buffers: Vec<&'a mut [u8]>) -> Self
	{
		let capacity = receive_buffers.len();

		let mut received_message_helpers = Vec::with_capacity(capacity);
		let mut multi_message_headers = Vec::with_capacity(capacity);
		for receive_buffer in receive_buffers
		{
			let mut received_message_helper = ReceivedMessageHelper::new(receive_buffer);
			let multi_message_header = received_message_helper.new_multi_message_header();
			multi_message_headers.push(multi_message_header);
			received_message_helpers.push(received_message_helper);
		}

		Self
		{
			received_message_helpers,
			multi_message_headers,
		}
	}

	/// Accesses a received message; only panics if `index` is too large when debug assertions are enabled.
	///
	/// The first field returned contains the bytes of the message received.
	/// The second field returned contains the remote address of the peer the message was received from.
	/// The first field returned contains whether this was an SCTP-like end-of-record (`true` if so).
	pub fn received_message_unchecked<'s: 'a>(&'s self, index: usize) -> Result<(&'a [u8], &'s SD, bool), ()>
	{
		debug_assert!(index < self.received_message_helpers.len(), "index `{}` is larger than self.message_helpers.len() `{}`", index, self.received_message_helpers.len());
		
		let multi_message_header = self.multi_message_headers.get_unchecked_safe(index);
		
		let flags = multi_message_header.msg_hdr.msg_flags;
		
		let is_sctp_like_end_of_record = if likely!(flags == 0)
		{
			false
		}
		else if likely!(flags == MSG_EOR)
		{
			true
		}
		else
		{
			return Err(())
		};
		
		let length = multi_message_header.msg_len as usize;
		
		let received_message_helper = self.received_message_helpers.get_unchecked_safe(index);
		
		Ok((&received_message_helper.receive_buffer[0 .. length], &received_message_helper.remote_peer_address, is_sctp_like_end_of_record))
	}

	/// Capacity.
	#[inline(always)]
	pub fn capacity(&self) -> usize
	{
		self.received_message_helpers.len()
	}
}
