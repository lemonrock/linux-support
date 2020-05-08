// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A receive message.
#[derive(Debug)]
pub struct ReceiveMessage<'a, SD: 'a + SocketData>
{
	internal: msghdr,

	// Owns lifetime of `msg_name` and `msg_namelen` in `internal`.
	pending_accept_connection: &'a PendingAcceptConnection<SD>,

	// Owns lifetime of `msg_iov` and source of `msg_iovlen` in `internal`.
	buffers: &'a [&'a mut [u8]],

	// Owns lifetime of `msg_control` and source of `msg_controllen` in `internal`.
	message_control: &'a mut [u8],
}

impl<'a, SD: 'a + SocketData> ReceiveMessage<'a, SD>
{
	#[allow(deprecated)]
	#[inline(always)]
	pub fn new(pending_accept_connection: &'a mut PendingAcceptConnection<SD>, buffers: &'a [&'a mut [u8]], message_control: &'a mut [u8]) -> Self
	{
		let message_iovlength = buffers.len();
		debug_assert!(message_iovlength <= i32::MAX as usize);

		let message_control_length = message_control.len();
		debug_assert!(message_control_length <= u32::MAX as usize);

		Self
		{
			internal: msghdr::new
			(
				&mut pending_accept_connection.peer_address as *mut SD as *mut c_void,
				pending_accept_connection.peer_address_length,
				buffers.as_ptr() as *mut iovec,
				message_iovlength as u32 as i32,
				message_control.as_mut_ptr() as *mut c_void,
				message_control_length as u32,
				unsafe { uninitialized() },
			),
			pending_accept_connection,
			buffers,
			message_control
		}
	}
}
