// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive buffer size.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ReceiveBufferSizeSocketOption
{
	/// Size.
	pub size: ReceiveBufferSizeInBytes,
	
	/// If the process has the capability `CAP_NET_ADMIN`, it can force the buffer `size` to be larger than `ReceiveBufferSizeSocketOption::global_maximum()`.
	#[serde(default)] pub force: bool,
}

impl ReceiveBufferSizeSocketOption
{
	#[inline(always)]
	pub(crate) fn adjusted_tcp_buffer_size(&self) -> i32
	{
		self.size.adjust_for_tcp_set_sock_opt() as c_int
	}
	
	#[inline(always)]
	pub(crate) fn socket_option_name(&self) -> i32
	{
		if self.force
		{
			SO_RCVBUFFORCE
		}
		else
		{
			SO_RCVBUF
		}
	}
}
