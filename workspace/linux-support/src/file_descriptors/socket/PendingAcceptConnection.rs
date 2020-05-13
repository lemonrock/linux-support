// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pending connection.
#[derive(Debug)]
pub struct PendingAcceptConnection<SD: SocketData>
{
	pub(crate) peer_address: SD,
	pub(crate) peer_address_length: socklen_t,
}

impl<SD: SocketData> PendingAcceptConnection<SD>
{
	/// New.
	#[allow(deprecated)]
	#[inline(always)]
	pub fn new() -> Self
	{

		Self
		{
			peer_address: unsafe { uninitialized() },
			peer_address_length: Self::SocketDataLength(),
		}
	}
	
	// Rust bug (as of 1.30) prevents this being a constant.
	#[inline(always)]
	pub(crate) fn SocketDataLength() -> socklen_t
	{
		size_of::<SD>() as socklen_t
	}
}
