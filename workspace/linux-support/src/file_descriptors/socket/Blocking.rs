// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Blocking, non-blocking or timing out.
///
/// Defaults to `Blocking { BlocksForever, BlocksForever }`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Blocking
{
	/// Non-blocking.
	NonBlocking,
	
	/// Blocking.
	Blocking
	{
		/// Send blocking duration.
		send: BlockingDuration,
		
		/// Receive blocking duration.
		receive: BlockingDuration,
	}
}

impl Default for Blocking
{
	#[inline(always)]
	fn default() -> Self
	{
		Blocking::Blocking
		{
			send: BlockingDuration::default(),
			receive: BlockingDuration::default(),
		}
	}
}

impl Blocking
{
	#[inline(always)]
	fn is_non_blocking(&self) -> bool
	{
		self == &Blocking::NonBlocking
	}
	
	#[inline(always)]
	fn set_time_outs(&self, socket_file_descriptor: &SocketFileDescriptor<impl SocketData>)
	{
		if let Blocking::Blocking { ref send, ref receive } = self
		{
			send.set_time_out_assuming_never_set_before(socket_file_descriptor, SO_SNDTIMEO);
			receive.set_time_out_assuming_never_set_before(socket_file_descriptor, SO_RCVTIMEO);
		}
	}
}
