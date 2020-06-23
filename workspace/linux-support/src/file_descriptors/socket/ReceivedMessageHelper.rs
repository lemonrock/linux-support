// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


pub(crate) struct ReceivedMessageHelper<'a, SD: SocketData>
{
	pub(crate) receive_buffer: &'a mut [u8],
	pub(crate) remote_peer_address: SD,
}

impl<'a, SD: SocketData> ReceivedMessageHelper<'a, SD>
{
	#[inline(always)]
	pub(crate) fn new(receive_buffer: &'a mut [u8]) -> Self
	{
		Self
		{
			receive_buffer,
			remote_peer_address: unsafe { zeroed() },
		}
	}

	#[inline(always)]
	pub(crate) fn new_multi_message_header(&mut self) -> mmsghdr
	{
		unsafe
		{
			let c_iovec =
			{
				// A Rust slice is actually an anonymous struct of `{ pointer_to_array: *mut T, length: usize }`; this is exactly the same layout as a C `iovec`.
				let c_iovec: &mut iovec = transmute(&mut self.receive_buffer);
				c_iovec
			};

			#[allow(deprecated)]
			mmsghdr
			{
				msg_hdr: msghdr::new(&mut self.remote_peer_address as *mut _ as *mut _, size_of::<SD>() as u32, c_iovec, 1, null_mut(), 0, uninitialized()),
				msg_len: uninitialized(),
			}
		}
	}
}
